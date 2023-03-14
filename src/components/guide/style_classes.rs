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
        <GuideWrapper cur_chapter="/guide/style-classes">
            <div class:section>
                <h2 class:section_title> "Define style classes" </h2>
                <p class:section_desc>
                    r#"Maomi supports a limited form of CSS."#
                </p>
                <p class:section_desc>
                    r#"In most cases, styles should be specified through classes."#
                </p>
                <CodeSegment>
                    <CodeLine em=&{Emphasize} text=r#"stylesheet! {"# />
                    <CodeLine em=&{Emphasize} text=r#"    class my_class {"# />
                    <CodeLine em=&{Emphasize} text=r#"        color = red;"# />
                    <CodeLine em=&{Emphasize} text=r#"    }"# />
                    <CodeLine em=&{Emphasize} text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <div class:my_class> "My Website" </div>"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Like common rust structs, classes can be public and used by multiple components."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine em=&{Emphasize} text=r#"    pub(crate) class my_class {"# />
                    <CodeLine text=r#"        color = red;"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"The class name is in the same namespace of the rust structs and enums."#
                </p>
                <p class:section_desc>
                    r#"Do not use element tag name as class names, otherwise the element tag name cannot be used."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeComment em=&{Wrong} indent=4 text=&{ trans!(r#"error: do not use "div" as class name"#) } />
                    <CodeLine em=&{Wrong} text=r#"    class div {"# />
                    <CodeLine text=r#"        color = red;"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Property syntax" </h2>
                <p class:section_desc>
                    r#"The stylesheet syntax is different from standard CSS."#
                </p>
                <p class:section_desc>
                    r#"The basic property syntax is "xxx = xxx;", and the CSS words should be connected with "_"."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine text=r#"    class my_class {"# />
                    <CodeComment em=&{Emphasize} indent=8 text=&{ trans!(r#"use "=" instead of ":", and ";" is always required"#) } />
                    <CodeLine em=&{Emphasize} text=r#"        color = red;"# />
                    <CodeComment em=&{Emphasize} indent=8 text=&{ trans!(r#"use "_" instead of "-""#) } />
                    <CodeLine em=&{Emphasize} text=r#"        background_color = rgb(192, 192, 192);"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Numbers with units should be written as follows:"#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine text=r#"    class my_class {"# />
                    <CodeComment em=&{Emphasize} indent=8 text=&{ trans!(r#"this generates "font-size: 1.5em;""#) } />
                    <CodeLine em=&{Emphasize} text=r#"        font_size = Em(1.5);"# />
                    <CodeComment em=&{Emphasize} indent=8 text=&{ trans!(r#"this generates "padding: 1px 0.1rem;""#) } />
                    <CodeLine em=&{Emphasize} text=r#"        padding = Px(1) Rem(0.1);"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r##"Colors in "#xxx" forms should be written as follows:"##
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine text=r#"    class my_class {"# />
                    <CodeComment em=&{Emphasize} indent=8 text=&{ trans!(r#"this generates "background-color: #00d2ff;""#) } />
                    <CodeLine em=&{Emphasize} text=r#"        background_color = Color("00d2ff");"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Debug the CSS output" </h2>
                <p class:section_desc>
                    r##"When not sure about the CSS output, add a "#[error_css_output]" before the class and the generated CSS will output as an error message."##
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine em=&{Emphasize} text=r#"    #[error_css_output]"# />
                    <CodeLine text=r#"    class my_class {"# />
                    <CodeLine text=r#"        background_color = Color("00d2ff");"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Hashed class names" </h2>
                <p class:section_desc>
                    r#"The class names are properly hashed. It is totally OK to use the same class name in different modules."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine text=r#"    class my_class {"# />
                    <CodeLine text=r#"        color = red;"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"mod another_mod {"# />
                    <CodeLine text=r#"    stylesheet! {"# />
                    <CodeLine text=r#"        class my_class {"# />
                    <CodeLine text=r#"            color = yellow;"# />
                    <CodeLine text=r#"        }"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"It is OK to specified the hashed name."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine text=r#"    #[css_name("my-class-1")]"# />
                    <CodeLine text=r#"    class my_class {"# />
                    <CodeLine text=r#"        color = red;"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Generated CSS output" </h2>
                <p class:section_desc>
                    r#"Maomi will generate a static CSS file for each crate."#
                </p>
                <p class:section_desc>
                    r#"To get the output CSS file, specify the output path in Cargo.toml."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"[package.metadata.maomi]"# />
                    <CodeLine text=r#"css-out-dir = "pkg""# />
                    <CodeLine text=r#"css-out-mode = "release""# />
                </_>
                <p class:section_desc>
                    r#"If "css-out-mode" is configured to "debug", the generated CSS will be easier to read."#
                </p>
                <p class:section_desc>
                    r#"The CSS output is in the specified "css-out-dir". Generated file name is "[CRATE_NAME].css". Use it in HTML head:"#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"<link rel="stylesheet" href="pkg/my_website.css">"# />
                </_>
                <p class:section_desc>
                    r#"This two options can be override by environment variables "MAOMI_CSS_OUT_DIR" and "MAOMI_CSS_OUT_MODE"."#
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
        trans!("{} - {}", trans!("maomi Guide"), trans!("Style Classes")).to_locale_string()
    }
}
