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
        <GuideWrapper cur_chapter="/guide/style-functions">
            <div class:section>
                <h2 class:section_title> "Functions with arguments" </h2>
                <p class:section_desc>
                    r#"To reuse similar properties in different classes, functions can be used."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine em=&{Emphasize} text=r#"    fn paddings() {"# />
                    <CodeLine em=&{Emphasize} text=r#"        padding = Px(1) Px(2);"# />
                    <CodeLine em=&{Emphasize} text=r#"    }"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"    class my_class {"# />
                    <CodeLine text=r#"        color = red;"# />
                    <CodeLine em=&{Emphasize} text=r#"        paddings();"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"    class my_another_class {"# />
                    <CodeLine em=&{Emphasize} text=r#"        paddings();"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Functions can take arguments, but the arguments can only be numbers (as "f32") or strings (as "&str")"#
                </p>
                <p class:section_desc>
                    r#"Number arguments can be used in unit-values."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine em=&{Emphasize} text=r#"    fn paddings(v: f32, h: f32) {"# />
                    <CodeLine em=&{Emphasize} text=r#"        padding = Px(v) Px(h);"# />
                    <CodeLine em=&{Emphasize} text=r#"    }"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"    class my_class {"# />
                    <CodeLine text=r#"        color = red;"# />
                    <CodeLine em=&{Emphasize} text=r#"        paddings(1, 2);"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"String arguments can be used in colors."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine em=&{Emphasize} text=r#"    fn text_color(c: &str) {"# />
                    <CodeLine em=&{Emphasize} text=r#"        color = Color(c);"# />
                    <CodeLine em=&{Emphasize} text=r#"    }"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"    class my_class {"# />
                    <CodeLine em=&{Emphasize} text=r#"        text_color("00d2ff");"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Dynamic Styling" </h2>
                <p class:section_desc>
                    r#"Besides classes, a special kind of functions can also be used in templates."#
                </p>
                <p class:section_desc>
                    r#"This kind of functions should contain exactly one argument, and declared with "style" instead of "fn"."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"stylesheet! {"# />
                    <CodeLine em=&{Emphasize} text=r#"    style text_color(c: &str) {"# />
                    <CodeLine em=&{Emphasize} text=r#"        color = Color(c);"# />
                    <CodeLine em=&{Emphasize} text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <div style:text_color=&{ self.color }> "My Website" </div>"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    color: String,"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"This can be used when some property values should be dynamically calculated."#
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
        trans!("{} - {}", trans!("maomi Guide"), trans!("Style Functions")).to_locale_string()
    }
}
