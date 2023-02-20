use maomi::{prelude::*, locale_string::{LocaleString, LocaleStaticStr}};
use maomi_dom::{prelude::*, element::*};

stylesheet! {
    use crate::*;

    const MAIN_COLOR: value = rgb(64, 64, 64);
    const COMMENT_COLOR: value = rgb(128, 128, 128);

    class wrapper {
        display = flex;
        font_family = "consolas", monospace;
        max_width = 100%;
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
        color = main_color;
        margin = 0 Em(0.1);
    }
    class comment {
        color = COMMENT_COLOR;
    }
}

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

// implement basic component interfaces
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
        <div class:line> { &Self::filtered_text(&self.text) } </div>
    },
    pub text: Prop<String>,
}

// implement basic component interfaces
impl Component for CodeLine {
    fn new() -> Self {
        Self {
            template: Default::default(),
            text: Default::default(),
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
        <div class:line class:comment> { &self.text } </div>
    },
    pub text: Prop<LocaleStaticStr>,
}

// implement basic component interfaces
impl Component for CodeComment {
    fn new() -> Self {
        Self {
            template: Default::default(),
            text: Default::default(),
        }
    }
}
