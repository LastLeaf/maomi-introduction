use maomi::locale_string::LocaleString;
use maomi::{prelude::*};
use maomi_dom::{prelude::*, element::*};

use crate::PageMeta;
use crate::components;
use components::page_wrapper::PageWrapper;
use components::utils::code_segment::*;
use components::utils::link::RawLink;

dom_css!(
    @import "/global.mcss";

    @macro text_style {
        () => {
            font-weight: normal;
        };
    }
    @const $main_text_color: rgb(32, 32, 32);
    @const $desc_color: rgb(64, 64, 64);
    @const $note_color: rgb(128, 128, 128);

    .title_section {
        margin: 40.px 0;
    }
    .title_icon {
        display: block;
        width: 160.px;
        margin: 0 auto;
    }
    .title {
        text_style!();
        text-align: center;
        font-size: 2.em;
        letter-spacing: 0.25.em;
        margin: 0.5.em;
        color: $icon_main;
    }
    .subtitle {
        text_style!();
        text-align: center;
        font-size: 1.2.em;
        margin: 0 0 20.px;
        color: $desc_color;
    }
    .title_code_wrapper {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .section {
        margin: 30.px auto;
        max-width: 620.px;
    }
    .section_title {
        text_style!();
        font-size: 1.3.em;
        margin: 0 0 0.5.em;
        color: $icon_main;
    }
    .section_desc {
        text_style!();
        color: $desc_color;
        margin: 0 0 0.5.em;
    }
    .section_img {}
    .section_note {
        font-size: 0.7.em;
        color: $note_color;
        margin: 0.5.em 0 0;
    }
    .code_wrapper {}
);

// declare a component
#[component(Backend = DomBackend, Translation = index)]
pub(crate) struct Index {
    template: template! {
        <PageWrapper>
            <div class:title_section>
                <img src="/res/icon_360.png" class:title_icon />
                <h1 class:title> "maomi" </h1>
                <p class:subtitle> "Strict and Performant Web Application Programing" </p>
                <div class:title_code_wrapper>
                    <CodeSegment>
                        <CodeLine text=r#"#[component]"# />
                        <CodeLine text=r#"struct HelloWorld {"# />
                        <CodeLine text=r#"    template: template! {"# />
                        <CodeLine text=r#"        "Hello world!""# />
                        <CodeLine text=r#"    }"# />
                        <CodeLine text=r#"}"# />
                    </_>
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "Programing in Rust" </h2>
                <div class:section_desc>
                    "Write rust code, compile to WebAssembly, and run in browser."
                </div>
                <div class:code_wrapper>
                    <CodeSegment>
                        <CodeComment text=&{ i18n!(r#"// declare a component"#) } />
                        <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                        <CodeLine text=r#"struct HelloWorld {"# />
                        <CodeComment text=&{ i18n!(r#"    // write a template"#) } />
                        <CodeLine text=r#"    template: template! {"# />
                        <CodeLine text=r#"        <div class:hello>"# />
                        <CodeLine text=r#"            "Hello world!""# />
                        <CodeLine text=r#"        </div>"# />
                        <CodeLine text=r#"    }"# />
                        <CodeLine text=r#"}"# />
                        <CodeLine text=r#""# />
                        <CodeComment text=&{ i18n!(r#"// implement component trait"#) } />
                        <CodeLine text=r#"impl Component for HelloWorld {"# />
                        <CodeLine text=r#"    fn new() -> Self {"# />
                        <CodeLine text=r#"        Self {"# />
                        <CodeLine text=r#"            template: Default::default()"# />
                        <CodeLine text=r#"        }"# />
                        <CodeLine text=r#"    }"# />
                        <CodeLine text=r#"}"# />
                        <CodeLine text=r#""# />
                        <CodeComment text=&{ i18n!(r#"// write styles"#) } />
                        <CodeLine text=r#"dom_css! {"# />
                        <CodeLine text=r#"    .hello {"# />
                        <CodeLine text=r#"        color: rgb(232, 152, 86);"# />
                        <CodeLine text=r#"    }"# />
                        <CodeLine text=r#"}"# />
                    </_>
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "Better Performance" </h2>
                <div class:section_desc>
                    "Maomi is highly optimized. The performance is even better than hand-written JavaScript without any framework."
                </div>
                <img class:section_img src="" />
                <div class:section_note>
                    "This DOM manipulation timing benchmark is based on "
                    <RawLink underline new_page url="https://github.com/krausest/js-framework-benchmark">
                        "js-framework-benchmark"
                    </RawLink>
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "Report Mistakes while Compilation" </h2>
                <div class:section_desc>
                    "Like rust, maomi reports mistakes while compilation. The mistakes include wrong element names, invalid properties, and even unmatched style class names."
                </div>
                <img class:section_img src="" />
            </div>
            <div class:section>
                <h2 class:section_title> "Work with rust-analyzer in IDE" </h2>
                <div class:section_desc>
                    "With rust-analyzer installed, it is much easier to investigate elements, properties, and even style classes."
                </div>
                <img class:section_img src="" />
            </div>
            <div class:section>
                <h2 class:section_title> "Data Bindings" </h2>
                <div class:section_desc>
                    "Maomi is based on templating and data bindings."
                </div>
                <div class:code_wrapper>
                    <CodeSegment>
                        <CodeLine text=r#"#[component]"# />
                        <CodeLine text=r#"struct HelloWorld {"# />
                        <CodeLine text=r#"    template: template! {"# />
                        <CodeLine text=r#"        <div class:hello>"# />
                        <CodeComment text=&{ i18n!(r#"            // use struct fields in the template"#) } />
                        <CodeLine text=r#"            { &self.hello }"# />
                        <CodeLine text=r#"        </div>"# />
                        <CodeLine text=r#"    },"# />
                        <CodeLine text=r#"    hello: String,"# />
                        <CodeLine text=r#"}"# />
                    </_>
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "Limited CSS" </h2>
                <div class:section_desc>
                    "Maomi supports a limited CSS-like syntax. It restricts the usage of CSS to make the styling easier to investigate."
                </div>
                <div class:code_wrapper>
                    <CodeSegment>
                        <CodeLine text=r#"dom_css! {"# />
                        <CodeLine text=r#"    .hello {"# />
                        <CodeLine text=r#"        color: rgb(232, 152, 86);"# />
                        <CodeLine text=r#"        :hover {"# />
                        <CodeLine text=r#"            text-decoration: underline;"# />
                        <CodeLine text=r#"        }"# />
                        <CodeLine text=r#"    }"# />
                        <CodeLine text=r#"}"# />
                    </_>
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "High Performance Server Side Rendering" </h2>
                <div class:section_desc>
                    "Maomi supports server side rendering. It just execute native rust binary in the server to generate HTML output. It is much more performant than using a JavaScript runtime."
                </div>
                <img class:section_img src="" />
            </div>
            <div class:section>
                <h2 class:section_title> "Integrated i18n Support" </h2>
                <div class:section_desc>
                    "Maomi supports i18n in the core design. It is easy to compile with TOML-based translation files to generate different version of the application."
                </div>
                <img class:section_img src="" />
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
