use maomi::{locale_string::*, prelude::*};
use maomi_dom::{element::*, prelude::*};

use crate::components;
use crate::PageMeta;
use components::page_wrapper::PageWrapper;

stylesheet!(
    use crate::*;

    class wrapper {
        text_align = center;
    }
);

#[component(Backend = DomBackend)]
pub(crate) struct NotFound {
    template: template! {
        <PageWrapper>
            <div class:wrapper> "Not Found" </div>
        </PageWrapper>
    },
}

impl Component for NotFound {
    fn new() -> Self {
        Self {
            template: Default::default(),
        }
    }
}

#[async_trait]
impl PrerenderableComponent for NotFound {
    type QueryData = ();
    type PrerenderingData = ();

    async fn prerendering_data(_: &Self::QueryData) -> Self::PrerenderingData {
        ()
    }

    fn apply_prerendering_data(&mut self, _: Self::PrerenderingData) {
        ()
    }
}

impl PageMeta for NotFound {
    fn title(&self) -> LocaleString {
        i18n!("Not Found").to_locale_string()
    }
}
