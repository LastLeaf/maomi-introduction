use maomi::{prelude::*, locale_string::*};
use maomi_dom::{prelude::*, element::*};

use crate::{PageMeta, components::utils::code_segment::*};
use super::{GuideWrapper, section, section_title, section_desc};

i18n_group!(guide as trans);

stylesheet!(
    use crate::*;
);

#[component(Backend = DomBackend, Translation = guide)]
pub(crate) struct Content {
    template: template! {
        <GuideWrapper cur_chapter="/guide/template-branches">
            <div class:section>
                <h2 class:section_title> "Quoted text" </h2>
                <p class:section_desc>
                    r#"The template looks like HTML or XML, but the text content must be quoted!"#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeComment em=&{Wrong} indent=8 text=&{ trans!(r#"Error: text content must be quoted"#) } />
                    <CodeLine em=&{Wrong} text=r#"        <div> My Website </div>"# />
                    <CodeComment em=&{Correct} indent=8 text=&{ trans!(r#"Correct:"#) } />
                    <CodeLine em=&{Correct} text=r#"        <div> "My Website" </div>"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
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
        trans!("{} - {}", trans!("maomi Guide"), trans!("Template Semantics")).to_locale_string()
    }
}
