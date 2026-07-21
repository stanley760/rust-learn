use std::{borrow::Cow, sync::{LazyLock, RwLock}};


use anyhow::Context;
use tera::Tera;

#[derive(Clone, Debug)]
pub struct Prompt {
    template_ref: TemplateRef,
    context: Option<tera::Context>,
}

#[derive(Clone, Debug)]
enum TemplateRef {
    OneOff(Cow<'static, str>),
    Tera(Cow<'static, str>),
}


pub(crate) fn default_prompt_template() -> Prompt {
    include_str!("system_prompt_template.md").into()
}

impl From<&'static str> for Prompt {
    fn from(prompt: &'static str) -> Self {
        Prompt {
            template_ref: TemplateRef::OneOff(prompt.into()),
            context: None,
        }
    }
}

impl From<String> for Prompt {
    fn from(prompt: String) -> Self {
        Prompt {
            template_ref: TemplateRef::OneOff(prompt.into()),
            context: None,
        }
    }
}

pub static TERA: LazyLock<RwLock<Tera>> = LazyLock::new(|| RwLock::new(Tera::default()));


impl Prompt {

    pub fn extend(other: &Tera) {
        let mut swiftide_tera = TERA.write().unwrap();
        swiftide_tera.register_from(other);
    }

     pub fn from_compiled_template(name: impl Into<Cow<'static, str>>) -> Prompt {
        Prompt {
            template_ref: TemplateRef::Tera(name.into()),
            context: None,
        }
    }

    #[must_use]
    pub fn with_context(mut self, new_context: impl Into<tera::Context>) -> Self {
        let context = self.context.get_or_insert_with(tera::Context::default);
        context.extend(new_context.into());

        self
    }

    #[must_use]
    pub fn with_context_value(mut self, key: &'static str, value: impl Into<tera::Value>) -> Self {
        let context = self.context.get_or_insert_with(tera::Context::default);
        context.insert(key, &value.into());
        self
    }

    pub fn render(&self) -> anyhow::Result<String> {
        if self.context.is_none() &&
            let TemplateRef::OneOff(ref template) = self.template_ref 
        {
            return Ok(template.to_string());
        }

        let context = self.context
            .as_ref()
            .map_or_else(|| Cow::Owned(tera::Context::default()), Cow::Borrowed);

        match &self.template_ref {
            TemplateRef::OneOff(template) => 
                tera::Tera::one_off(template.as_ref(), &context, false)
                    .context("Failed to render one-off template"),
            TemplateRef::Tera(template) => TERA
                .read()
                .unwrap()
                .render(template.as_ref(), &context)
                .context("Failed to render tera template"),
        }
    }
}