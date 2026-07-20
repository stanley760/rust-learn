use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;
use strum::VariantArray;
use strum_macros::{AsRefStr, Display, EnumString, VariantArray};
use walkdir::WalkDir;

pub const MEMORY_INDEX_FILE: &str = "MEMORY.md";
pub const MAX_INDEX_LINES: usize = 200;
pub const MEMORY_GUIDANCE: &str = r#"
When to save memories:
- User states a preference ("I like tabs", "always use pytest") -> type: user
- User corrects you ("don't do X", "that was wrong because...") -> type: feedback
- You learn a project fact that is not easy to infer from current code alone
  (for example: a rule exists because of compliance, or a legacy module must
  stay untouched for business reasons) -> type: project
- You learn where an external resource lives (ticket board, dashboard, docs URL)
  -> type: reference

When NOT to save:
- Anything easily derivable from code (function signatures, file structure, directory layout)
- Temporary task state (current branch, open PR numbers, current TODOs)
- Secrets or credentials (API keys, passwords)
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, VariantArray, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum Type {
    User,
    Feedback,
    Project,
    Reference,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub name: String,
    pub description: String,
    pub memory_type: Type,
    pub content: String,
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "### {}: {}", self.name, self.description)?;
        if !self.content.is_empty() {
            writeln!(f, "{}", self.content.trim())?;
        }

        Ok(())
    }
}

pub struct Manager {
    dir: PathBuf,
    memories: HashMap<String, Entry>,
}

pub fn get_memory_manager(dir: PathBuf) -> Result<Manager> {
    let mut manager = Manager::new(dir);
    manager.load_all()?;
    Ok(manager)
}

impl Manager {
    pub fn new(dir: PathBuf) -> Self {
        Self {
            dir,
            memories: HashMap::new(),
        }
    }

    pub fn load_all(&mut self) -> Result<()> {
        self.memories.clear();

        if !self.dir.exists() {
            return Ok(());
        }

        for entry in WalkDir::new(&self.dir)
            .max_depth(1)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("md"))
            .filter(|entry| entry.file_name().to_str() != Some(MEMORY_INDEX_FILE))
        {
            let path = entry.path();
            let content = std::fs::read_to_string(path)
                .with_context(|| format!("can't read memory file: {}", path.display()))?;

            let Some(parsed) = parse_frontmatter(&content)? else {
                continue;
            };

            let name = parsed.name.unwrap_or_else(|| {
                path.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into()
            });

            let description = parsed.description.unwrap_or_default();

            let memory_type = parsed
                .memory_type
                .as_deref()
                .unwrap_or("project")
                .parse::<Type>()?;

            self.memories.insert(
                name.clone(),
                Entry {
                    name,
                    description,
                    memory_type,
                    content: parsed.content,
                },
            );
        }

        Ok(())
    }

    pub fn load_memory_prompt(&self) -> String {
        if self.memories.is_empty() {
            return String::new();
        }

        let mut lines = vec![
            "# Memories(persistent across sessions)".to_string(),
            String::new(),
        ];

        for memory_type in Type::VARIANTS {
            let mut typed = self
                .memories
                .values()
                .filter(|entry| entry.memory_type == *memory_type)
                .collect::<Vec<_>>();
            typed.sort_by(|e1, e2| e1.name.cmp(&e2.name));

            if typed.is_empty() {
                continue;
            }
            lines.push(format!("## [{}]", memory_type));

            for entry in typed {
                lines.push(entry.to_string());
                lines.push(String::new());
            }
        }

        lines.join("\n").trim().to_string()
    }

    pub fn save_memory(
        &mut self,
        name: &str,
        description: &str,
        memory_type: Type,
        content: &str,
    ) -> Result<String> {
        let safe_name = sanitize_name(name);
        if safe_name.is_empty() {
            return Err(anyhow::anyhow!("invalid memory name"));
        }

        std::fs::create_dir_all(&self.dir)
            .with_context(|| format!("can't create memory dir: {}", self.dir.display()))?;

        let file_name = format!("{safe_name}.md");
        let file_path = self.dir.join(&file_name);
        let frontmatter = format!(
            "---\nname: {name}\ndescription:{description}\ntype:{memory_type}\n---\n{content}\n"
        );
        std::fs::write(&file_path, frontmatter)
            .with_context(|| format!("can't write memory file: {}", file_path.display()))?;

        self.memories.insert(
            name.to_string(),
            Entry {
                name: name.to_string(),
                description: description.to_string(),
                memory_type,
                content: content.to_string(),
            },
        );
        self.rebuild_index()?;
        Ok(format!(
            "Save memory '{}' [{}] to {}",
            name,
            memory_type,
            file_path
                .strip_prefix(
                    std::env::current_dir()
                        .unwrap_or_else(|_| std::path::PathBuf::from("."))
                )
                .unwrap_or(&file_path)
                .display()
        ))
    }

    pub fn memories(&self) -> &HashMap<String, Entry> {
        &self.memories
    }

    pub fn describe_memories(&self) -> String {
        if self.memories.is_empty() {
            return "  (no memories)".to_string();
        }

        self.sorted_memories()
            .into_iter()
            .map(|entry| {
                format!(
                    " [{}] {}: {}",
                    entry.memory_type, entry.name, entry.description
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn rebuild_index(&self) -> Result<()> {
        let mut lines = vec!["# Memory Index".to_string(), String::new()];
        for entry in self.sorted_memories() {
            lines.push(format!(
                "- {}:{} [{}]",
                entry.name, entry.description, entry.memory_type
            ));

            if lines.len() > MAX_INDEX_LINES {
                lines.push(format!("... (truncated at {} lines)", MAX_INDEX_LINES));
                break;
            }
        }
        let index_path = self.dir.join(MEMORY_INDEX_FILE);
        std::fs::write(&index_path, format!("{}\n", lines.join("\n")))
            .with_context(|| format!("can't write memory index: {}", index_path.display()))?;
        Ok(())
    }

    fn sorted_memories(&self) -> Vec<&Entry> {
        let mut memories = self.memories.values().collect::<Vec<_>>();
        memories.sort_by(|m1, m2| m1.name.cmp(&m2.name));
        memories
    }
}

#[derive(Debug, Default, Deserialize)]
struct Formatter {
    name: Option<String>,
    description: Option<String>,
    #[serde(rename = "type")]
    memory_type: Option<String>,
}

struct ParsedMemory {
    name: Option<String>,
    description: Option<String>,
    memory_type: Option<String>,
    content: String,
}

fn parse_frontmatter(text: &str) -> Result<Option<ParsedMemory>> {
    let text = text.replace("\r\n", "\n");

    let Some(result) = text.strip_prefix("---\n") else {
        return Ok(None);
    };

    let Some((frontmatter, body)) = result.split_once("\n---\n") else {
        return Ok(None);
    };

    let meta = serde_yaml::from_str::<Formatter>(frontmatter).unwrap_or_default();

    Ok(Some(ParsedMemory {
        name: meta.name,
        description: meta.description,
        memory_type: meta.memory_type,
        content: body.trim().to_string(),
    }))
}

fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_string()
}
