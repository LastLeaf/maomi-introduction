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
        <GuideWrapper cur_chapter="/guide/component-events">
            <div class:section>
                <h2 class:section_title> "Using events in components" </h2>
                <p class:section_desc>
                    r#"Components can trigger events to its users."#
                </p>
                <p class:section_desc>
                    r#"An event can contain a piece of data "T", wrapped as "Event<T>". The event data should be provided when triggering it."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyChild {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine text=r#"        <div> "This line is in the child component." </div>"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine em=&{Emphasize} text=r#"    pub my_event: Event<String>,"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl Component for MyChild {"# />
                    <CodeLine text=r#"    fn new() -> Self {"# />
                    <CodeLine text=r#"        Self {"# />
                    <CodeLine text=r#"            template: Default::default(),"# />
                    <CodeLine text=r#"            my_event: Default::default(),"# />
                    <CodeLine text=r#"        }"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"    fn created(&self) {"# />
                    <CodeLine em=&{Emphasize} text=r#"        let mut data = "my event data".to_string();"# />
                    <CodeLine em=&{Emphasize} text=r#"        self.my_event.trigger(&mut data);"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <MyChild my_event=@handle_my_event() />"# />
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
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl MyWebsite {"# />
                    <CodeLine em=&{Emphasize} text=r#"    fn handle_my_event(this: ComponentEvent<Self, String>) {"# />
                    <CodeLine em=&{Emphasize} text=r#"        log::debug!("{}", this.detail());"# />
                    <CodeLine em=&{Emphasize} text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Note that the event fields should be properly "pub" so that it can be assigned from templates in other rust modules."#
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
        trans!("{} - {}", trans!("maomi Guide"), trans!("Component Events")).to_locale_string()
    }
}
