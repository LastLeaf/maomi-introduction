use maomi::{prelude::*, locale_string::LocaleString};
use maomi_dom::{prelude::*, element::*};

use crate::PageMeta;
use super::{GuideWrapper, section, section_title, section_desc};

stylesheet!(
    use crate::*;
);

#[component(Backend = DomBackend)]
pub(crate) struct Content {
    template: template! {
        <GuideWrapper cur_chapter_path="/guide">
            <div class:section>
                <h2 class:section_title> "#[component] struct" </h2>
                <div class:section_desc>
                    "Maomi is optimized for speed. The performance is even better than hand-written JavaScript without any framework."
                </div>
            </div>
        </GuideWrapper>
    },
}

impl Component for Content {
    fn new() -> Self {
        Self {
            template: Default::default(),
        }
    }
}

#[async_trait]
impl PrerenderableComponent for Content {
    type QueryData = ();
    type PrerenderingData = ();

    async fn prerendering_data(_: &Self::QueryData) -> Self::PrerenderingData {
        ()
    }

    fn apply_prerendering_data(&mut self, _: Self::PrerenderingData) {
        ()
    }
}

impl PageMeta for Content {
    fn title(&self) -> LocaleString {
        i18n!(guide, "Write a Component").to_locale_string()
    }
}
