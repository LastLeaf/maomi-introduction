use maomi::prelude::*;
use maomi_dom::{prelude::*, element::*};

use crate::components;
use components::utils::link::*;

stylesheet! {
    use crate::*;

    class header_wrapper {
        box_sizing = border_box;
        height = Px(100);
        font_size = Em(1.2);
        line_height = Px(100);
        white_space = nowrap;
        padding = 0 Px(20);
        border_bottom = Px(5) solid ICON_MAIN;
        color = ICON_MAIN;
        display = flex;
        overflow = hidden;
        background_color = white;
    }
    class icon {
        height = 100%;
        margin_right = Px(20);
        if media (max_width = Px(600)) {
            margin_right = Px(5);
        }
        if media (max_width = Px(540)) {
            display = none;
        }
    }
    class title {
        flex = 1;
        height = 100%;
        font_size = Em(1.2);
        letter_spacing = Em(0.2);
        padding = 0 Px(20);
        if media (max_width = Px(800)) {
            display = none;
        }
    }
    class header_link {
        padding = 0 Px(20);
        margin_left = Px(20);
        if media (max_width = Px(600)) {
            padding_left = Px(10);
            padding_right = Px(10);
            margin_left = Px(5);
        }
        if hover {
            background = ICON_SUB;
        }
    }

    class main {
        padding = Px(20) Px(10);
        color = TEXT_MAIN;
        background_color = white;
    }

    class footer_wrapper {
        padding = Px(10) Px(20);
        border_top = Px(5) solid ICON_MAIN;
        color = rgb(128, 128, 128);
        text_align = center;
        font_size = Em(0.9);
        background = ICON_SUB;
    }
    class footer_row {
        margin = Em(0.5) 0;
    }
}

#[component(Backend = DomBackend)]
pub(crate) struct PageWrapper {
    template: template! {
        <header class:header_wrapper>
            <img src=&{ crate::config::res_path("icon_360.png") } class:icon />
            <div class:title> "maomi" </div>
            <div class:header_link>
                <Link path="/">
                    <div> "Introduction" </div>
                </Link>
            </div>
            <div class:header_link>
                <Link path="/guide">
                    <div> "Guide" </div>
                </Link>
            </div>
            <div class:header_link>
                <RawLink new_page url="https://docs.rs/maomi/latest/maomi/">
                    <div> "Documents" </div>
                </RawLink>
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
            <div class:footer_row>
                "Copyright 2023 lastleaf.cn | "
                <RawLink new_page url="https://beian.miit.gov.cn/"> "粤ICP备17044902号-2" </RawLink>
            </div>
        </footer>
    },
}

impl Component for PageWrapper {
    fn new() -> Self {
        Self {
            template: Default::default(),
        }
    }
}
