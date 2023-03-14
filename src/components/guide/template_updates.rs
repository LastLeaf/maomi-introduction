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
        <GuideWrapper cur_chapter="/guide/template-updates">
            <div class:section>
                <h2 class:section_title> "Template Updates" </h2>
                <p class:section_desc>
                    r#"Maomi is based on data bindings. Changing the component fields will auto-update the template."#
                </p>
                <p class:section_desc>
                    r#"However, changes to the component fields should always be asynchronous. To visit the component fields, a task must be generated."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine text=r#"        <div> "My Website" </div>"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    my_string: String,"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl Component for MyWebsite {"# />
                    <CodeLine text=r#"    fn new() -> Self {"# />
                    <CodeLine text=r#"        Self {"# />
                    <CodeLine text=r#"            template: Default::default(),"# />
                    <CodeLine text=r#"            my_string: "original".to_string(),"# />
                    <CodeLine text=r#"        }"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#""# />
                    <CodeComment em=&{Emphasize} indent=4 text=&{ trans!(r#"this function is executed after the component creation"#) } />
                    <CodeLine em=&{Emphasize} text=r#"    fn created(&self) {"# />
                    <CodeComment em=&{Emphasize} indent=8 text=&{ trans!(r#"acquire a ref-counted token of the component"#) } />
                    <CodeLine em=&{Emphasize} text=r#"        let this = self.rc();"# />
                    <CodeComment em=&{Emphasize} indent=8 text=&{ trans!(r#"generate a task"#) } />
                    <CodeLine em=&{Emphasize} text=r#"        this.task(move |this| {"# />
                    <CodeLine em=&{Emphasize} text=r#"            this.my_string = "changed".to_string();"# />
                    <CodeComment em=&{Emphasize} indent=12 text=&{ trans!(r#"the template will be updated after the task ends"#) } />
                    <CodeLine em=&{Emphasize} text=r#"        });"# />
                    <CodeLine em=&{Emphasize} text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "About Tasks" </h2>
                <p class:section_desc>
                    r#"Tasks are asynchronous jobs that can visit one component."#
                </p>
                <p class:section_desc>
                    r#"To generate a task, call task on the ref-counted token. In this way, the template will always be updated after the task ends."#
                </p>
                <p class:section_desc>
                    r#"Sometimes it is needed to read fields but not update them. To avoid the template update overhead, it is able to generate a read-or-write task."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"impl Component for MyWebsite {"# />
                    <CodeLine text=r#"    fn created(&self) {"# />
                    <CodeLine text=r#"        let this = self.rc();"# />
                    <CodeLine text=r#"        this.task_with(move |this, ctx| {"# />
                    <CodeComment em=&{Emphasize} indent=12 text=&{ trans!(r#"the template will be not be updated"#) } />
                    <CodeComment em=&{Emphasize} indent=12 text=&{ trans!(r#"unless need_update is called"#) } />
                    <CodeLine em=&{Emphasize} text=r#"            // ctx.need_update();"# />
                    <CodeLine text=r#"        });"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Because tasks are asynchronous, references cannot move across tasks. Thus &self is not usable in tasks. The ref-counted token can be cloned and move across tasks."#
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
        trans!("{} - {}", trans!("maomi Guide"), trans!("Template Updates")).to_locale_string()
    }
}
