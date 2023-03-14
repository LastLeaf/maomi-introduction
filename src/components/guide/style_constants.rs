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
        <GuideWrapper cur_chapter="/guide/style-constants">
            <div class:section>
                <h2 class:section_title> "Constant value" </h2>
                <p class:section_desc>
                    r#"It is able to define constants in stylesheets."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine em=&{Emphasize} text=r#"    const TEXT_COLOR: value = rgb(32, 32, 32);"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"    class my_class {"# />
                    <CodeLine em=&{Emphasize} text=r#"        color = TEXT_COLOR;"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"The constant can be any CSS token, but should only be a single token."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeComment em=&{Wrong} indent=4 text=&{ trans!(r#"cannot be multiple tokens"#) } />
                    <CodeLine em=&{Wrong} text=r#"    const SIZE: value = Px(1) Px(2);"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Keyframes definition" </h2>
                <p class:section_desc>
                    r#"Keyframes can be declared as a special type of constants."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine em=&{Emphasize} text=r#"    const MY_ANIMATION: keyframes = {"# />
                    <CodeLine em=&{Emphasize} text=r#"        from {"# />
                    <CodeLine em=&{Emphasize} text=r#"            opacity = 1;"# />
                    <CodeLine em=&{Emphasize} text=r#"        }"# />
                    <CodeLine em=&{Emphasize} text=r#"        50% {"# />
                    <CodeLine em=&{Emphasize} text=r#"            opacity = 0.5;"# />
                    <CodeLine em=&{Emphasize} text=r#"        }"# />
                    <CodeLine em=&{Emphasize} text=r#"        to {"# />
                    <CodeLine em=&{Emphasize} text=r#"            opacity = 1;"# />
                    <CodeLine em=&{Emphasize} text=r#"        }"# />
                    <CodeLine em=&{Emphasize} text=r#"    };"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"    class my_class {"# />
                    <CodeLine em=&{Emphasize} text=r#"        animation_name = MY_ANIMATION;"# />
                    <CodeLine em=&{Emphasize} text=r#"        animation_duration = S(2);"# />
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
        trans!("{} - {}", trans!("maomi Guide"), trans!("Style Constants")).to_locale_string()
    }
}
