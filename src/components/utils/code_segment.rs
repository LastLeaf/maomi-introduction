use maomi::{prelude::*, locale_string::{LocaleString, LocaleStaticStr}};
use maomi_dom::{prelude::*, element::*};

dom_css! {
    @import "/global.mcss";

    @const $main_color: rgb(64, 64, 64);
    @const $comment_color: rgb(128, 128, 128);

    .wrapper {
        display: flex;
        font-family: "consolas", monospace;
        max-width: 100%;
    }
    .left {
        width: 5.px;
        background-color: $icon_sub;
        flex: none;
    }
    .block {
        margin: 0.5.em 0 0.5.em 1.em;
        overflow: auto;
    }

    .line {
        white-space: pre;
        color: $main_color;
        margin: 0 0.1.em;
    }
    .comment {
        color: $comment_color;
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
