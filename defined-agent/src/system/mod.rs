mod prompt;

use derive_builder::Builder;

use crate::system::prompt::{Prompt, default_prompt_template};

#[derive(Debug, Clone, Builder)]
#[builder(setter(into, strip_option))]
pub struct SystemPrompt {
    #[builder(default)]
    role: Option<String>,
    #[builder(default)]
    skills_available: Option<String>,
    #[builder(default)]
    memory: Option<String>,
    #[builder(default)]
    claude_md: Option<String>,
    #[builder(default)]
    dynamic_context: Option<String>,
    #[builder(default)]
    memory_guidance: Option<String>,

    #[builder(default, setter(custom))]
    guidelines: Vec<String>,
    #[builder(default, setter(custom))]
    constraints: Vec<String>,

    #[builder(default)]
    additional: Option<String>,

    #[builder(default= default_prompt_template())]
    template: Prompt,
}

impl SystemPrompt {
    pub fn builder() -> SystemPromptBuilder {
        SystemPromptBuilder::default()
    }

    pub fn to_prompt(&self) -> Prompt {
        self.clone().into()
    }

    pub fn with_added_guideline(&mut self, guideline: impl AsRef<str>) -> &mut Self {
        self.guidelines.push(guideline.as_ref().to_string());
        self
    }

    pub fn with_added_constraint(&mut self, constraint: impl AsRef<str>) -> &mut Self {
        self.constraints.push(constraint.as_ref().to_string());
        self
    }

    pub fn with_guidelines<T: IntoIterator<Item = S>, S: AsRef<str>>(
        &mut self,
        guidlines: T,
    ) -> &mut Self {
        self.guidelines = guidlines
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect();
        self
    }

    pub fn with_constraints<T: IntoIterator<Item = S>, S: AsRef<str>>(
        &mut self,
        constraints: T,
    ) -> &mut Self {
        self.constraints = constraints
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect();
        self
    }

    pub fn with_role(&mut self, role: impl Into<String>) -> &mut Self {
        self.role = Some(role.into());
        self
    }

    pub fn with_skills_available(&mut self, skills_available: impl Into<String>) -> &mut Self {
        self.skills_available = Some(skills_available.into());
        self
    }

    /// Sets the memory section.
    pub fn with_memory(&mut self, memory: impl Into<String>) -> &mut Self {
        self.memory = Some(memory.into());
        self
    }

    /// Sets the CLAUDE.md section.
    pub fn with_claude_md(&mut self, claude_md: impl Into<String>) -> &mut Self {
        self.claude_md = Some(claude_md.into());
        self
    }

    /// Sets the dynamic context section.
    pub fn with_dynamic_context(&mut self, dynamic_context: impl Into<String>) -> &mut Self {
        self.dynamic_context = Some(dynamic_context.into());
        self
    }

    /// Sets the memory guidance section.
    pub fn with_memory_guidance(&mut self, guidance: impl Into<String>) -> &mut Self {
        self.memory_guidance = Some(guidance.into());
        self
    }

    /// Sets the additional markdown field.
    pub fn with_additional(&mut self, additional: impl Into<String>) -> &mut Self {
        self.additional = Some(additional.into());
        self
    }

    /// Sets the template.
    pub fn with_template(&mut self, template: impl Into<Prompt>) -> &mut Self {
        self.template = template.into();
        self
    }
}

impl SystemPromptBuilder {
    pub fn add_guideline(&mut self, guideline: &str) -> &mut Self {
        self.guidelines
            .get_or_insert_with(Vec::new)
            .push(guideline.to_string());
        self
    }

    pub fn add_constraint(&mut self, constraint: &str) -> &mut Self {
        self.constraints
            .get_or_insert_with(Vec::new)
            .push(constraint.to_string());
        self
    }

    pub fn guidelines<T: IntoIterator<Item = S>, S: AsRef<str>>(
        &mut self,
        guidlines: T,
    ) -> &mut Self {
        self.guidelines = Some(
            guidlines
                .into_iter()
                .map(|s| s.as_ref().to_string())
                .collect(),
        );
        self
    }

    pub fn constraints<T: IntoIterator<Item = S>, S: AsRef<str>>(
        &mut self,
        constraints: T,
    ) -> &mut Self {
        self.constraints = Some(
            constraints
                .into_iter()
                .map(|s| s.as_ref().to_string())
                .collect(),
        );
        self
    }
}

impl From<Prompt> for SystemPrompt {
    fn from(prompt: Prompt) -> Self {
        SystemPrompt {
            role: None,
            skills_available: None,
            memory: None,
            claude_md: None,
            dynamic_context: None,
            memory_guidance: None,
            guidelines: Vec::new(),
            constraints: Vec::new(),
            additional: None,
            template: prompt,
        }
    }
}

impl Default for SystemPrompt {
    fn default() -> Self {
        SystemPrompt {
            role: None,
            skills_available: None,
            memory: None,
            claude_md: None,
            dynamic_context: None,
            memory_guidance: None,
            guidelines: Vec::new(),
            constraints: Vec::new(),
            additional: None,
            template: default_prompt_template(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Prompt> for SystemPrompt {
    fn into(self) -> Prompt {
        let SystemPrompt {
            role,
            skills_available,
            memory,
            claude_md,
            dynamic_context,
            memory_guidance,
            guidelines,
            constraints,
            additional,
            template,
        } = self;

        template
            .with_context_value("role", role)
            .with_context_value("skills_available", skills_available)
            .with_context_value("memory", memory)
            .with_context_value("claude_md", claude_md)
            .with_context_value("dynamic_context", dynamic_context)
            .with_context_value("memory_guidance", memory_guidance)
            .with_context_value("guidelines", guidelines)
            .with_context_value("constraints", constraints)
            .with_context_value("additional", additional)
    }
}
