use maomi::prelude::*;
use maomi_dom::{prelude::*, element::*};

use crate::components;
use components::utils::link::*;

dom_css! {
    @import "/global.mcss";

    .header_wrapper {
        box-sizing: border-box;
        height: 100.px;
        font-size: 1.2.em;
        line-height: 100.px;
        white-space: nowrap;
        padding: 0 20.px;
        border-bottom: 5.px solid $icon_main;
        color: $icon_main;
        display: flex;
    }
    .icon {
        height: 100%;
        margin-right: 20.px;
        @media (max-width: 600.px) {
            margin-right: 5.px;
        }
    }
    .title {
        flex: 1;
        height: 100%;
        font-size: 1.2.em;
        letter-spacing: 0.2.em;
        padding: 0 20.px;
        @media (max-width: 800.px) {
            display: none;
        }
    }
    .header_link {
        padding: 0 20.px;
        margin-left: 20.px;
        @media (max-width: 600.px) {
            padding-left: 10.px;
            padding-right: 10.px;
            margin-left: 5.px;
        }
        :hover {
            background: $icon_sub;
        }
    }

    .main {
        margin: 20.px 0;
        padding: 0 10.px;
    }

    .footer_wrapper {
        padding: 10.px 20.px;
        border-top: 5.px solid $icon_main;
        color: rgb(128, 128, 128);
        text-align: center;
        font-size: 0.9.em;
        background: $icon_sub;
    }
    .footer_row {
        margin: 0.5.em 0;
    }
}

#[component(Backend = DomBackend)]
pub(crate) struct PageWrapper {
    template: template! {
        <header class:header_wrapper>
            <img src="/res/icon_360.png" class:icon />
            <div class:title> "maomi" </div>
            <div class:header_link>
                <Link path="/">
                    <div> "Introduction" </div>
                </Link>
            </div>
            <div class:header_link>
                <Link path="/quick-start">
                    <div> "Quick Start" </div>
                </Link>
            </div>
            <div class:header_link>
                <Link path="/documents">
                    <div> "Documents" </div>
                </Link>
            </div>
        </header>
        <div class:main>
            <slot />
        </div>
        <footer class:footer_wrapper>
            <div class:footer_row>
                <RawLink underline new_page url="https://github.com/LastLeaf/maomi">
                    "GitHub"
                </RawLink>
                " | "
                <RawLink underline new_page url="https://crates.io/crates/maomi">
                    "crates.io"
                </RawLink>
                " | "
                <RawLink underline new_page url="http://lastleaf.me">
                    "lastleaf.me"
                </RawLink>
            </div>
            <div class:footer_row>
                "This website is powered by maomi. "
                <RawLink underline new_page url="https://github.com/LastLeaf/maomi-introduction"> "Source code is here." </RawLink>
            </div>
        </footer>
    },
}

// implement basic component interfaces
impl Component for PageWrapper {
    fn new() -> Self {
        Self {
            template: Default::default(),
        }
    }
}

impl PageWrapper {
}
