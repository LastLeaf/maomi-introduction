use maomi::prelude::*;
use maomi_dom::{prelude::*, element::*};

use crate::components;
use components::utils::link::Link;

dom_css! {
    @const $header_height: 100.px;

    .icon {
        height: $header_height;
    }
}

#[component(Backend = DomBackend)]
pub(crate) struct PageWrapper {
    template: template! {
        <header>
            <img src="/res/icon.png" class:icon />
            <div> "maomi" </div>
            <Link path="/">
                <div> "Introduction" </div>
            </Link>
            <Link path="/quick-start">
                <div> "Quick Start" </div>
            </Link>
            <Link path="/documents">
                <div> "Documents" </div>
            </Link>
        </header>
        <div>
            <slot />
        </div>
        <footer>
            <a href="https://github.com/LastLeaf/maomi">
                <div> "GitHub" </div>
            </a>
            "This website is powered by maomi. "
            <a href="https://github.com/LastLeaf/maomi-introduction"> "Source code is here." </a>
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
