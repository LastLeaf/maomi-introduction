use maomi::locale_string::LocaleString;
use maomi::{prelude::*};
use maomi_dom::{prelude::*, element::*};

use crate::PageMeta;
use crate::components;
use components::page_wrapper::PageWrapper;

dom_css!(
    .section {
        padding: 20.px;
    }

    .section_title {
        font_size: 1.5.em;
    }

    .section_img {}
    .section_desc {}
);

// declare a component
#[component(Backend = DomBackend, Translation = index)]
pub(crate) struct Index {
    template: template! {
        <PageWrapper>
            <h1> "maomi" </h1>
            <div class:section>
                <h2> "Strict and Performant Web Application Programing" </h2>
                <h2 class:section_title> "Better Performance" </h2>
                <img class:section_img src="" />
                <div class:section_desc>
                    ""
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "Programing in Rust" </h2>
                <img class:section_img src="" />
                <div class:section_desc>
                    ""
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "Report Mistakes while Compilation" </h2>
                <img class:section_img src="" />
                <div class:section_desc>
                    ""
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "Work with rust-analyzer in IDE" </h2>
                <img class:section_img src="" />
                <div class:section_desc>
                    ""
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "Limited CSS" </h2>
                <img class:section_img src="" />
                <div class:section_desc>
                    ""
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "High Performance Server Side Rendering" </h2>
                <img class:section_img src="" />
                <div class:section_desc>
                    ""
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "Integrated i18n Support" </h2>
                <img class:section_img src="" />
                <div class:section_desc>
                    ""
                </div>
            </div>
        </PageWrapper>
    },
}

// implement basic component interfaces
impl Component for Index {
    fn new() -> Self {
        Self {
            template: Default::default(),
        }
    }
}

impl Index {
}

#[async_trait]
impl PrerenderableComponent for Index {
    type QueryData = ();
    type PrerenderingData = ();

    async fn prerendering_data(_: &Self::QueryData) -> Self::PrerenderingData {
        ()
    }

    fn apply_prerendering_data(&mut self, _: Self::PrerenderingData) {
        ()
    }
}

impl PageMeta for Index {
    fn title(&self) -> LocaleString {
        i18n!(index, "maomi - Strict and Performant Web Application Programing").to_locale_string()
    }
}
