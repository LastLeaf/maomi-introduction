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
        <GuideWrapper cur_chapter="/guide/properties">
            <div class:section>
                <h2 class:section_title> "Using properties in components" </h2>
                <p class:section_desc>
                    r#"Components can contain several fields which are assignable through template attributes."#
                </p>
                <p class:section_desc>
                    r#"The properties should be wrapped with "Prop". The "Prop" type implements "Deref" so it can be dereferenced to the contained type."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyChild {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine text=r#"        <div> { format!("The content is {}", *self.content) } </div>"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine em=&{Emphasize} text=r#"    pub content: Prop<String>,"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl Component for MyChild {"# />
                    <CodeLine text=r#"    fn new() -> Self {"# />
                    <CodeLine text=r#"        Self {"# />
                    <CodeLine text=r#"            template: Default::default(),"# />
                    <CodeLine em=&{Emphasize} text=r#"            content: Prop::new("default".to_string()),"# />
                    <CodeLine text=r#"        }"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <MyChild content="from MyWebsite" />"# />
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
                    r#"Note that the property fields should be properly "pub" so that it can be assigned from templates in other rust modules."#
                </p>
            </div>

            <div class:section>
                <h2 class:section_title> "Two-way properties" </h2>
                <p class:section_desc>
                    r#"Common properties are one-way: the component receives the value from its user."#
                </p>
                <p class:section_desc>
                    r#"Sometimes it is required to be two-way, e.g. synchronize input values between components."#
                </p>
                <p class:section_desc>
                    r#"In these cases, "BindingProp" and "BindingValue" can be used. However, the changes of two-way values do not automatically triggering the template update. A task should be manually generated if the template update is required."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"use maomi::prop::{BindingProp, BindingValue};"# />
                    <CodeLine text=r#"use maomi_dom::event::*;"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyChild {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine text=r#"        <input value=&{ self.input_value } input=@value_changed() />"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    input_value: BindingValue<String>,"# />
                    <CodeLine text=r#"    pub content: BindingProp<String>,"# />
                    <CodeLine text=r#"    pub change: Event<()>,"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl Component for MyChild {"# />
                    <CodeLine text=r#"    fn new() -> Self {"# />
                    <CodeLine text=r#"        Self {"# />
                    <CodeLine text=r#"            template: Default::default(),"# />
                    <CodeLine text=r#"            input_value: Default::default(),"# />
                    <CodeLine text=r#"            content: Default::default(),"# />
                    <CodeLine text=r#"            change: Default::default(),"# />
                    <CodeLine text=r#"        }"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl MyChild {"# />
                    <CodeLine text=r#"    fn value_changed(this: ComponentRc<Self>, _: &mut InputEvent) {"# />
                    <CodeLine text=r#"        this.task_with(|this, _| {"# />
                    <CodeLine text=r#"            this.content.set(this.input_value.get());"# />
                    <CodeLine text=r#"            this.change.trigger(&mut ());"# />
                    <CodeLine text=r#"        });"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine text=r#"        <MyChild content=&{ self.content } change=@content_changed() />"# />
                    <CodeLine text=r#"        <div> { self.content.get() } </div>"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    content: BindingValue<String>,"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl Component for MyWebsite {"# />
                    <CodeLine text=r#"    fn new() -> Self {"# />
                    <CodeLine text=r#"        Self {"# />
                    <CodeLine text=r#"            template: Default::default(),"# />
                    <CodeLine text=r#"            content: Default::default(),"# />
                    <CodeLine text=r#"        }"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl MyWebsite {"# />
                    <CodeLine text=r#"    fn content_changed(this: ComponentRc<Self>, _: &mut ()) {"# />
                    <CodeLine text=r#"        this.task(|_| {});"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Note that "BindingProp" and "BindingValue" do not implement "Deref" - use explicit getter functions to retrieve the value."#
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
        trans!("{} - {}", trans!("maomi Guide"), trans!("Properties")).to_locale_string()
    }
}
