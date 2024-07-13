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
        <GuideWrapper cur_chapter="/guide/events">
            <div class:section>
                <h2 class:section_title> "Event bindings" </h2>
                <p class:section_desc>
                    r#"It is able to bind an event listener function to DOM events."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"use maomi_dom::event::*;"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <div click=@handle_click()> "Click Me" </div>"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl MyWebsite {"# />
                    <CodeLine text=r#"    fn handle_click(this: ComponentRc<Self>, _ev: &mut MouseEvent) {"# />
                    <CodeLine text=r#"        this.task(move |this| {"# />
                    <CodeLine text=r#"            // update component here"# />
                    <CodeLine text=r#"        })"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Custom event arguments" </h2>
                <p class:section_desc>
                    r#"When binding listeners, extra arguments can be added."#
                </p>
                <p class:section_desc>
                    r#"Only references can be used as arguments (and the data type should implement `Clone` or `ToOwned` trait)."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"use maomi_dom::event::*;"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <div click=@handle_click(&self.my_string_data)> "Click Me" </div>"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    my_string_data: String,"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl MyWebsite {"# />
                    <CodeLine em=&{Emphasize} text=r#"    fn handle_click(_this: ComponentRc<Self>, _ev: &mut MouseEvent, s: &str) {"# />
                    <CodeLine text=r#"        log::debug!("Clicked: {}", s);"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Tap events" </h2>
                <p class:section_desc>
                    r#"Maomi provides tap events as a combination of mouse events and touch events."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <div tap=@handle_tap()> "Click Me" </div>"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl MyWebsite {"# />
                    <CodeLine text=r#"    fn handle_tap(_this: ComponentRc<Self>, _ev: &mut TapEvent) {"# />
                    <CodeLine text=r#"        log::debug!("Tapped");"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Long tap events are also supported."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <div tap=@handle_tap() long_tap=@handle_long_tap()> "Click Me" </div>"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl MyWebsite {"# />
                    <CodeLine text=r#"    fn handle_tap(_this: ComponentRc<Self>, _ev: &mut TapEvent) {"# />
                    <CodeLine text=r#"        log::debug!("Tapped");"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"    fn handle_long_tap(_this: ComponentRc<Self>, ev: &mut TapEvent) {"# />
                    <CodeComment em=&{Emphasize} indent=8 text=&{ trans!(r#"avoid further tap event"#) } />
                    <CodeLine em=&{Emphasize} text=r#"        ev.prevent_default();"# />
                    <CodeLine text=r#"        log::debug!("Long tapped");"# />
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
        trans!("{} - {}", trans!("maomi Guide"), trans!("Events")).to_locale_string()
    }
}
