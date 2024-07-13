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
        <GuideWrapper cur_chapter="/guide/template-branches">
            <div class:section>
                <h2 class:section_title> r#"If statements"# </h2>
                <p class:section_desc>
                    r#"The if statements are allowed."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        if self.my_boolean_data {"# />
                    <CodeLine text=r#"            <div> "My Optional Element" </div>"# />
                    <CodeLine em=&{Emphasize} text=r#"        } else {"# />
                    <CodeLine text=r#"            <div> "Another Element" </div>"# />
                    <CodeLine em=&{Emphasize} text=r#"        }"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    my_boolean_data: bool,"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Likewise, the if-let statements are also allowed."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        if let Some(s) = &self.my_option {"# />
                    <CodeLine text=r#"            <div> { s } </div>"# />
                    <CodeLine em=&{Emphasize} text=r#"        }"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    my_option: Option<String>,"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>
            <div class:section>
                <h2 class:section_title> r#"Match statements"# </h2>
                <p class:section_desc>
                    r#"The match statements are just like common rust code."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        match &self.my_option {"# />
                    <CodeLine em=&{Emphasize} text=r#"            Some(s) => {"# />
                    <CodeLine text=r#"                <div> { s } </div>"# />
                    <CodeLine em=&{Emphasize} text=r#"            }"# />
                    <CodeLine em=&{Emphasize} text=r#"            None => {"# />
                    <CodeLine text=r#"                <div> "Another Element" </div>"# />
                    <CodeLine em=&{Emphasize} text=r#"            }"# />
                    <CodeLine em=&{Emphasize} text=r#"        }"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    my_option: Option<String>,"# />
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
            trans!("Template Branches")
        )
        .to_locale_string()
    }
}
