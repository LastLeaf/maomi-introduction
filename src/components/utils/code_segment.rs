use maomi::{
    locale_string::{LocaleStaticStr, LocaleString},
    prelude::*,
};
use maomi_dom::{element::*, prelude::*};

stylesheet! {
    use crate::*;

    const COMMENT_COLOR: value = rgb(128, 128, 128);
    const EMPHASIZE_COLOR: value = rgb(0, 107, 202);
    const CORRECT_COLOR: value = rgb(0, 155, 61);
    const ERROR_COLOR: value = rgb(217, 48, 9);

    class wrapper {
        display = flex;
        font_family = "consolas", monospace;
        max_width = 100%;
        color = TEXT_SUB;
    }
    class left {
        width = Px(5);
        background_color = ICON_SUB;
        flex = none;
    }
    class block {
        margin = Em(0.5) 0 Em(0.5) Em(1);
        overflow = auto;
    }

    class line {
        white_space = pre;
        color = TEXT_SUB;
        margin = 0 Em(0.1);
    }
    class comment {
        color = COMMENT_COLOR;
    }

    class emphasize {
        color = EMPHASIZE_COLOR;
    }
    class error {
        color = ERROR_COLOR;
    }
    class correct {
        color = CORRECT_COLOR;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum CodeEm {
    None,
    Emphasize,
    Correct,
    Wrong,
}

impl Default for CodeEm {
    fn default() -> Self {
        Self::None
    }
}

pub(crate) use CodeEm::{Correct, Emphasize, Wrong};

#[component(Backend = DomBackend)]
pub(crate) struct CodeSegment {
    template: template! {
        <div class:wrapper>
            <div class:left></div>
            <div class:block>
                <slot />
            </div>
        </div>
    },
}

impl Component for CodeSegment {
    fn new() -> Self {
        Self {
            template: Default::default(),
        }
    }
}

#[component(Backend = DomBackend)]
pub(crate) struct CodeLine {
    template: template! {
        <div
            class:line
            class:emphasize=&{ *self.em == CodeEm::Emphasize }
            class:correct=&{ *self.em == CodeEm::Correct }
            class:error=&{ *self.em == CodeEm::Wrong }
        > { &Self::filtered_text(&self.text) } </div>
    },
    pub text: Prop<String>,
    pub em: Prop<CodeEm>,
}

impl Component for CodeLine {
    fn new() -> Self {
        Self {
            template: Default::default(),
            text: Default::default(),
            em: Default::default(),
        }
    }
}

impl CodeLine {
    fn filtered_text(text: &str) -> LocaleString {
        if text.len() == 0 {
            LocaleString::translated(" ")
        } else {
            LocaleString::translated(&text)
        }
    }
}

#[component(Backend = DomBackend)]
pub(crate) struct CodeComment {
    template: template! {
        <div
            class:line
            class:comment
            class:emphasize=&{ *self.em == CodeEm::Emphasize }
            class:correct=&{ *self.em == CodeEm::Correct }
            class:error=&{ *self.em == CodeEm::Wrong }
        > { &self.whole_text } </div>
    },
    pub text: Prop<LocaleStaticStr>,
    pub indent: Prop<u32>,
    pub em: Prop<CodeEm>,
    whole_text: LocaleString,
}

impl Component for CodeComment {
    fn new() -> Self {
        Self {
            template: Default::default(),
            text: Default::default(),
            indent: Default::default(),
            em: Default::default(),
            whole_text: Default::default(),
        }
    }

    fn before_template_apply(&mut self) {
        let mut s = String::new();
        for _ in 0..*self.indent {
            s += " ";
        }
        self.whole_text = LocaleString::translated(format!("{}// {}", s, self.text));
    }
}
