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
        <GuideWrapper cur_chapter="/guide/runtime-performance-tips">
            <div class:section>
                <h2 class:section_title> r#"About execution speed"# </h2>
                <p class:section_desc>
                    r#"Maomi is greatly optimized on execution speed. In most cases, there is no need to worry about it."#
                </p>
                <p class:section_desc>
                    r#"On the other hand, most high-level code does not need to be so fast. Don't be afraid to write simple code that is a little slower."#
                </p>
                <p class:section_desc>
                    r#"If there is a need of profiling, note that the release builds are much faster then debug builds (like most rust programs do)."#
                </p>
            </div>
            <div class:section>
                <h2 class:section_title> r#"About code size"# </h2>
                <p class:section_desc>
                    r#"In most cases, WebAssembly binary is bigger than JavaScript code, but it can have a better compression rate. Do not forget to turn on HTTP compression on the generated binary."#
                </p>
                <p class:section_desc>
                    r#"Besides that, some code generation options have great helps to reduce the generated WebAssembly binary size. It is recommended to use the following options in Cargo.toml."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"[profile.release]"# />
                    <CodeLine text=r#"opt-level = "z""# />
                    <CodeLine text=r#"lto = true"# />
                    <CodeLine text=r#"codegen-units = 1"# />
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
