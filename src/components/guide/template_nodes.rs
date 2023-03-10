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
        <GuideWrapper cur_chapter="/guide/template-nodes">
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

            <div class:section>
                <h2 class:section_title> "Comments" </h2>
                <p class:section_desc>
                    r#"Note that comments are rust-style (not XML style)."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        // This is a line of comments"# />
                    <CodeLine em=&{Emphasize} text=r#"        /* Some other comments */"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "End tags and self-close tags" </h2>
                <p class:section_desc>
                    r#"The end tag can be replaced by "</_>" for short."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <div> "My Website" </_>"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Self-close tags (like XML) are also supported."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <div />"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Dynamic text content" </h2>
                <p class:section_desc>
                    r#"It is able to use an expression as the text content. Use a brace "{}" to embed an expression."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <div> { format!("1 + 2 = {}", 1 + 2) } </div>"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"It is able to use "self" in the expression."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <div> { &self.my_data } </div>"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    my_data: String,"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine text=r#"        <div> "My Website" </div>"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    my_data: format!("1 + 2 = {}", 1 + 2),"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Be careful: when using fields in "self", do not move out the fields."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Wrong} text=r#"        <div> { self.my_data } </div>"# />
                    <CodeLine em=&{Correct} text=r#"        <div> { &self.my_data } </div>"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    my_data: String,"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Attributes" </h2>
                <p class:section_desc>
                    r#"It is able to use attributes (like XML), but non-string values should not be quoted."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <div id="a" />"# />
                    <CodeLine em=&{Emphasize} text=r#"        <div hidden=true />"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"The "=true" can be omitted."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <div hidden />"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Attributes also support expressions. Note that the attributes are typed."#
                </p>
                <p class:section_desc>
                    r#"String attributes can only be assigned string values. Boolean attributes can only be assigned boolean values."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Wrong} text=r#"        <div id={ &self.my_boolean_data } />"# />
                    <CodeLine em=&{Correct} text=r#"        <div id={ &self.my_string_data } />"# />
                    <CodeLine em=&{Wrong} text=r#"        <div hidden={ &self.my_string_data } />"# />
                    <CodeLine em=&{Correct} text=r#"        <div hidden={ &self.my_boolean_data } />"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    my_string_data: String,"# />
                    <CodeLine text=r#"    my_boolean_data: bool,"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Expressions for attributes always accept references."#
                </p>
                <p class:section_desc>
                    r#"The "&" can be written outside the brace - sometimes it looks better."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <div hidden=&{ !self.my_boolean_data } />"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    my_string_data: String,"# />
                    <CodeLine text=r#"    my_boolean_data: bool,"# />
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
