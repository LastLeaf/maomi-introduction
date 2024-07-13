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
        <GuideWrapper cur_chapter="/guide/template-repeats">
            <div class:section>
                <h2 class:section_title> r#"For statements"# </h2>
                <p class:section_desc>
                    r#"The for statements are allowed."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        for s in &self.my_list {"# />
                    <CodeLine text=r#"            <div> { s } </div>"# />
                    <CodeLine em=&{Emphasize} text=r#"        }"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    my_list: Vec<String>,"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>
            <div class:section>
                <h2 class:section_title> r#"For statements with keys"# </h2>
                <p class:section_desc>
                    r#"If the list often changes, it's better to specify a key for the list."#
                </p>
                <p class:section_desc>
                    r#"The key should be an unique identifier for each item. It is used to identify how the list changes and make list changes faster."#
                </p>
                <p class:section_desc>
                    r#"To specify a key, the list structure must implement "AsListKey"."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        for item in &self.my_list use u32 {"# />
                    <CodeLine text=r#"            <div> { &item.s } </div>"# />
                    <CodeLine text=r#"        }"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    my_list: Vec<MyListItem>,"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"struct MyListItem {"# />
                    <CodeLine text=r#"    id: u32,"# />
                    <CodeLine text=r#"    s: String,"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine em=&{Emphasize} text=r#"impl AsListKey for MyListItem {"# />
                    <CodeLine em=&{Emphasize} text=r#"    type ListKey = u32;"# />
                    <CodeLine em=&{Emphasize} text=r#""# />
                    <CodeLine em=&{Emphasize} text=r#"    fn as_list_key(&self) -> &u32 {"# />
                    <CodeLine em=&{Emphasize} text=r#"        &self.id"# />
                    <CodeLine em=&{Emphasize} text=r#"    }"# />
                    <CodeLine em=&{Emphasize} text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"If the for statement generates more than one variable, it is required to specify which variable is used as the key."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine text=r#"        for (_index, item) in self.my_list.iter().enumerate()"# />
                    <CodeLine em=&{Emphasize} text=r#"            use (item) u32"# />
                    <CodeLine text=r#"        {"# />
                    <CodeLine text=r#"            <div> { &item.s } </div>"# />
                    <CodeLine text=r#"        }"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    my_list: Vec<MyListItem>,"# />
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
        trans!("{} - {}", trans!("maomi Guide"), trans!("Template Repeats")).to_locale_string()
    }
}
