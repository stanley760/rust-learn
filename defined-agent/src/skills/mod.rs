use anyhow::{Context, Ok, Result};
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};
use walkdir::WalkDir;
mod skill_load;
pub use skill_load::load_skills_tool;

pub struct SkillManifest {
    pub name: String,
    pub description: String,
    pub path: PathBuf,
}

pub struct SkillDocument {
    pub manifest: SkillManifest,
    pub body: String,
}

impl std::fmt::Display for SkillDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<skill name="{}"> 
            {}
            </skill>"#,
            self.manifest.name, self.body
        )
    }
}

pub fn get_skill_registry(skills_dir: PathBuf) -> Result<SkillRegistry> {
    let mut registry = SkillRegistry::new(skills_dir);
    registry.load_skills()?;
    Ok(registry)
}

pub struct SkillRegistry {
    skills_dir: PathBuf,
    skills: HashMap<String, SkillDocument>,
}

impl SkillRegistry {
    pub fn new(skills_dir: PathBuf) -> Self {
        Self {
            skills_dir,
            skills: HashMap::new(),
        }
    }
    pub fn load_skills(&mut self) -> Result<()> {
        self.skills.clear();

        if !self.skills_dir.exists() {
            return Ok(());
        }
        for entry in WalkDir::new(&self.skills_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.file_name().to_str() == Some("SKILL.md"))
        {
            let path = entry.path();
            let content = std::fs::read_to_string(path)
                .with_context(|| format!("can't read skill file: {}", path.display()))?;
            let (meta, body) = parse_formatter(&content);

            let fallback_name = path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            let name = meta.name.unwrap_or(fallback_name);
            let description = meta
                .description
                .unwrap_or_else(|| "No description".to_string());

            let doc = SkillDocument {
                manifest: SkillManifest {
                    name: name.clone(),
                    description,
                    path: path.to_path_buf(),
                },
                body,
            };

            self.skills.insert(name, doc);
        }

        Ok(())
    }

    pub fn load_full_text(&self, name:&str) -> String {
        match self.skills.get(name) {
            Some(skill) => skill.to_string(),
            None => {
                let mut names = self.skills.keys().cloned().collect::<Vec<_>>();
                names.sort();
                format!(
                    "Error: Unknown skill '{}'. Available: {}",
                    name, 
                    names.join(", ")
                )
            }
        }
    }

    pub fn describe_available(&self) -> String {
        if self.skills.is_empty() {
            return "(no skill available)".to_string();
        }
        let mut names = self.skills.keys().cloned().collect::<Vec<_>>();
        names.sort();

        names.into_iter()
            .filter_map(|name| {
                self.skills.get(&name).map(|skill| {
                    format!(" - {}:{}", skill.manifest.name, skill.manifest.description)
                })
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn skills(&self) -> &HashMap<String, SkillDocument> {
        &self.skills
    }
}

#[derive(Debug, Default, Deserialize)]
struct SkillFormatter {
    name: Option<String>,
    description: Option<String>,
}

fn parse_formatter(text: &str) -> (SkillFormatter, String) {
    let text = text.replace("\r\n", "\n");
    let Some(res) = text.strip_prefix("---\n") else {
        return (SkillFormatter::default(), text.trim().to_string());
    };
    let Some((formatter, body)) = res.split_once("\n---\n") else {
        return (SkillFormatter::default(), text.trim().to_string());
    };

    let meta = serde_yaml::from_str::<SkillFormatter>(formatter).unwrap_or_default();

    (meta, body.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_formatter_with_yaml_frontmatter() {
        let input = "---\nname: my-skill\ndescription: A test skill\n---\nThis is the body content.";
        let (meta, body) = parse_formatter(input);
        assert_eq!(meta.name.as_deref(), Some("my-skill"));
        assert_eq!(meta.description.as_deref(), Some("A test skill"));
        assert_eq!(body, "This is the body content.");
    }

    #[test]
    fn test_parse_formatter_no_frontmatter() {
        let input = "Just a plain text body without any frontmatter.";
        let (meta, body) = parse_formatter(input);
        assert!(meta.name.is_none());
        assert!(meta.description.is_none());
        assert_eq!(body, "Just a plain text body without any frontmatter.");
    }

    #[test]
    fn test_parse_formatter_crlf_line_endings() {
        let input = "---\r\nname: crlf-skill\r\ndescription: CRLF test\r\n---\r\nBody with CRLF.";
        let (meta, body) = parse_formatter(input);
        assert_eq!(meta.name.as_deref(), Some("crlf-skill"));
        assert_eq!(meta.description.as_deref(), Some("CRLF test"));
        assert_eq!(body, "Body with CRLF.");
    }

    #[test]
    fn test_parse_formatter_incomplete_frontmatter_no_closing() {
        let input = "---\nname: incomplete\nThis is body without closing delimiter.";
        let (meta, _body) = parse_formatter(input);
        // No closing "---\n", so should fall back to default
        assert!(meta.name.is_none());
        assert!(meta.description.is_none());
    }

    #[test]
    fn test_parse_formatter_empty_frontmatter() {
        // "---\n---\n..." — after strip_prefix, remaining is "---\nBody...",
        // which does NOT contain "\n---\n", so it falls back to default
        let input = "---\n---\nBody after empty frontmatter.";
        let (meta, body) = parse_formatter(input);
        assert!(meta.name.is_none());
        assert!(meta.description.is_none());
        assert_eq!(body, "---\n---\nBody after empty frontmatter.");
    }

    #[test]
    fn test_parse_formatter_partial_frontmatter_only_name() {
        let input = "---\nname: partial-skill\n---\nBody here.";
        let (meta, body) = parse_formatter(input);
        assert_eq!(meta.name.as_deref(), Some("partial-skill"));
        assert!(meta.description.is_none());
        assert_eq!(body, "Body here.");
    }

    #[test]
    fn test_parse_formatter_invalid_yaml_falls_back_to_default() {
        let input = "---\n: invalid yaml [[[\n---\nBody content.";
        let (meta, body) = parse_formatter(input);
        assert!(meta.name.is_none());
        assert!(meta.description.is_none());
        assert_eq!(body, "Body content.");
    }

    #[test]
    fn test_parse_formatter_trims_whitespace() {
        let input = "  \n  ---\nname: trimmed\n---\n  Body with padding.  \n  ";
        let (meta, _body) = parse_formatter(input);
        // Leading whitespace before "---" means it won't match strip_prefix
        assert!(meta.name.is_none());
    }

    #[test]
    fn test_parse_formatter_body_with_multiple_lines() {
        let input = "---\nname: multi-line\n---\nLine 1\nLine 2\nLine 3";
        let (meta, body) = parse_formatter(input);
        assert_eq!(meta.name.as_deref(), Some("multi-line"));
        assert_eq!(body, "Line 1\nLine 2\nLine 3");
    }
}