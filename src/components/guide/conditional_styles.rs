use maomi::{locale_string::*, prelude::*};
use maomi_dom::{element::*, prelude::*};

use super::{section, section_desc, section_title, GuideWrapper};
use crate::{components::utils::code_segment::*, PageMeta};

i18n_group!(guide as trans);

stylesheet!(
    use crate::*;
);

#[component(Backend = DomBackend, Translation = guide)]
pub(crate) struct Content {
    template: template! {
        <GuideWrapper cur_chapter="/guide/conditional-styles">
            <div class:section>
                <h2 class:section_title> "Media Queries" </h2>
                <p class:section_desc>
                    r#"Unlike standard CSS, media queries should be inside classes."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine text=r#"    class my_class {"# />
                    <CodeLine text=r#"        width = Px(600);"# />
                    <CodeLine text=r#""# />
                    <CodeLine em=&{Emphasize} text=r#"        if media (max_width = Px(600)) and (min_width = Px(300)) {"# />
                    <CodeLine em=&{Emphasize} text=r#"            width = auto;"# />
                    <CodeLine em=&{Emphasize} text=r#"        }"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Feature detection with "@supports" likewise."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine text=r#"    class my_class {"# />
                    <CodeLine em=&{Emphasize} text=r#"        if supports (width = Vw(100)) {"# />
                    <CodeLine em=&{Emphasize} text=r#"            width = Vw(100);"# />
                    <CodeLine em=&{Emphasize} text=r#"        }"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Pseudo classes" </h2>
                <p class:section_desc>
                    r#"Pseudo classes should also be inside classes."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine text=r#"    class my_class {"# />
                    <CodeLine em=&{Emphasize} text=r#"        if hover {"# />
                    <CodeLine em=&{Emphasize} text=r#"            text_decoration = underline;"# />
                    <CodeLine em=&{Emphasize} text=r#"        }"# />
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
        trans!(
            "{} - {}",
            trans!("maomi Guide"),
            trans!("Conditional Styles")
        )
        .to_locale_string()
    }
}
