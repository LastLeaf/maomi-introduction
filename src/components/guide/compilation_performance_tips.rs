use maomi::{locale_string::*, prelude::*};
use maomi_dom::{element::*, prelude::*};

use super::{section, section_desc, section_title, GuideWrapper};
use crate::PageMeta;

i18n_group!(guide as trans);

stylesheet!(
    use crate::*;
);

#[component(Backend = DomBackend, Translation = guide)]
pub(crate) struct Content {
    template: template! {
        <GuideWrapper cur_chapter="/guide/compilation-performance-tips">
            <div class:section>
                <h2 class:section_title> r#"About compilation time"# </h2>
                <p class:section_desc>
                    r#"Maomi requires a lot of compile-time optimizations, so the compilation time might be really long in old machines."#
                </p>
                <p class:section_desc>
                    r#"To improve the compilation speed, the best way is to split components into several small crates, since rust compilation is crate-by-crate. If a crate code is not changed, it will not be re-compiled."#
                </p>
                <p class:section_desc>
                    r#"Another important tip is to avoid server side rendering compilation while debugging component logic. Since when server side rendering requires native binary for each components, this doubles the compilation time."#
                </p>
                <p class:section_desc>
                    r#"In general, smaller components bring more code re-using and compile faster. In maomi, components are near zero-cost abstractions which has very low runtime overhead."#
                </p>
            </div>
            <div class:section>
                <h2 class:section_title> r#"About working with IDE"# </h2>
                <p class:section_desc>
                    r#"When editing code in IDE with rust-analyzer, large code may slow down rust-analyzer, bringing bad coding experience."#
                </p>
                <p class:section_desc>
                    r#"It is able to tell maomi to optimize for rust-analyzer. When working with rust-analyzer, maomi can try to generate faster results. The results is not runnable but fast in cargo check."#
                </p>
                <p class:section_desc>
                    r#"To enable this experimental feature, set environment variable MAOMI_RUST_ANALYZER=on to rust-analyzer. Use Visual Studio Code as the example, this environment variable can be set in "rust-analyzer.cargo.extraEnv" and "rust-analyzer.check.extraEnv" settings."#
                </p>
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
            trans!("Compilation Performance Tips")
        )
        .to_locale_string()
    }
}
