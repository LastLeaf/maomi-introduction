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
        <GuideWrapper cur_chapter="/guide/using-components">
            <div class:section>
                <h2 class:section_title> "Using component in component" </h2>
                <p class:section_desc>
                    r#"Components in maomi are low-cost abstractions. Some web frameworks have a significant overhead with components, but maomi only has a little. Sometimes components can help improve performance."#
                </p>
                <p class:section_desc>
                    r#"Like DOM elements, a component can be used as tags."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyChild {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine text=r#"        <div> "This line is in the child component." </div>"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl Component for MyChild {"# />
                    <CodeLine text=r#"    fn new() -> Self {"# />
                    <CodeLine text=r#"        Self {"# />
                    <CodeLine text=r#"            template: Default::default(),"# />
                    <CodeLine text=r#"        }"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine text=r#"        <MyChild />"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl Component for MyWebsite {"# />
                    <CodeLine text=r#"    fn new() -> Self {"# />
                    <CodeLine text=r#"        Self {"# />
                    <CodeLine text=r#"            template: Default::default(),"# />
                    <CodeLine text=r#"        }"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Slots in components" </h2>
                <p class:section_desc>
                    r#"The component can put a "<slot />" tag in the template. It will be replaced with the child nodes in its users."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyChild {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine text=r#"        <div> "This line is in the child component." </div>"# />
                    <CodeLine em=&{Emphasize} text=r#"        <slot />"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl Component for MyChild {"# />
                    <CodeLine text=r#"    fn new() -> Self {"# />
                    <CodeLine text=r#"        Self {"# />
                    <CodeLine text=r#"            template: Default::default(),"# />
                    <CodeLine text=r#"        }"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <MyChild>"# />
                    <CodeLine em=&{Emphasize} text=r#"            <div> "This line will be placed in the slot." </div>"# />
                    <CodeLine em=&{Emphasize} text=r#"        </_>"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl Component for MyWebsite {"# />
                    <CodeLine text=r#"    fn new() -> Self {"# />
                    <CodeLine text=r#"        Self {"# />
                    <CodeLine text=r#"            template: Default::default(),"# />
                    <CodeLine text=r#"        }"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"It is able to pass data in the slot with "data" attribute. Its users can retrieve the reference of the data with "slot:xxx" attributes."#
                </p>
                <p class:section_desc>
                    r#"Besides it, the slot data type should be specified with the "SlotData" attribute argument."#
                </p>
                <CodeSegment>
                    <CodeLine em=&{Emphasize} text=r#"#[component(Backend = DomBackend, SlotData = String)]"# />
                    <CodeLine text=r#"struct MyChild {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine text=r#"        <div> "This line is in the child component." </div>"# />
                    <CodeLine em=&{Emphasize} text=r#"        <slot data=&{ self.my_string_data } />"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    my_string_data: String,"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl Component for MyChild {"# />
                    <CodeLine text=r#"    fn new() -> Self {"# />
                    <CodeLine text=r#"        Self {"# />
                    <CodeLine text=r#"            template: Default::default(),"# />
                    <CodeLine text=r#"            my_string_data: "slot data".to_string(),"# />
                    <CodeLine text=r#"        }"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <MyChild slot:sd>"# />
                    <CodeLine em=&{Emphasize} text=r#"            <div> { sd } </div>"# />
                    <CodeLine em=&{Emphasize} text=r#"        </_>"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl Component for MyWebsite {"# />
                    <CodeLine text=r#"    fn new() -> Self {"# />
                    <CodeLine text=r#"        Self {"# />
                    <CodeLine text=r#"            template: Default::default(),"# />
                    <CodeLine text=r#"        }"# />
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
        trans!("{} - {}", trans!("maomi Guide"), trans!("Using Components")).to_locale_string()
    }
}
