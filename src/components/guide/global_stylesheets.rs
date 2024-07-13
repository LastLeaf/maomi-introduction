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
        <GuideWrapper cur_chapter="/guide/global-stylesheets">
            <div class:section>
                <h2 class:section_title> "Global stylesheet usage" </h2>
                <p class:section_desc>
                    r#"Some constants may be used in several stylesheets in the whole crate. It is able to write them in a global stylesheet."#
                </p>
                <p class:section_desc>
                    r#"The default global stylesheet is located at "src/lib.mcss". This path can be changed by "stylesheet-mod-root" in Cargo.toml."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"[package.metadata.maomi]"# />
                    <CodeLine text=r#"stylesheet-mod-root = "src/lib.mcss""# />
                </_>
                <p class:section_desc>
                    r#"Constants can be declared inside this file."#
                </p>
                <CodeSegment>
                    <CodeLine em=&{Emphasize} text=r#"pub(crate) const TEXT_COLOR: value = rgb(32, 32, 32);"# />
                </_>
                <p class:section_desc>
                    r#"Stylesheets can use the global constants with a "use" statement."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine em=&{Emphasize} text=r#"    use crate::*;"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"    class my_class {"# />
                    <CodeLine text=r#"        color = TEXT_COLOR;"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Normal functions (not dynamic style functions) can also be declared."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"pub(crate) const TEXT_COLOR: value = rgb(32, 32, 32);"# />
                    <CodeLine em=&{Emphasize} text=r#"pub(crate) fn paddings() {"# />
                    <CodeLine em=&{Emphasize} text=r#"    padding = Px(1) Px(2);"# />
                    <CodeLine em=&{Emphasize} text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Then in stylesheets:"#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine em=&{Emphasize} text=r#"    use crate::*;"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"    class my_class {"# />
                    <CodeLine text=r#"        color = TEXT_COLOR;"# />
                    <CodeLine text=r#"        paddings();"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Submodules in global stylesheets" </h2>
                <p class:section_desc>
                    r#"If the global stylesheet is large, the "mod" statement can be used to split it into several files."#
                </p>
                <CodeSegment>
                    <CodeLine em=&{Emphasize} text=r#"pub(crate) mod my_sub;"# />
                </_>
                <p class:section_desc>
                    r#"In "src/my_sub.mcss":"#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"pub(crate) const TEXT_COLOR: value = rgb(32, 32, 32);"# />
                </_>
                <p class:section_desc>
                    r#"Then in stylesheets:"#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine em=&{Emphasize} text=r#"    use crate::my_sub::*;"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"    class my_class {"# />
                    <CodeLine text=r#"        color = TEXT_COLOR;"# />
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
            trans!("Global Stylesheets")
        )
        .to_locale_string()
    }
}
