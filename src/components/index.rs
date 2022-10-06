// import maomi core module
use maomi::{prelude::*, locale_string::*};
// using DOM backend
use maomi_dom::{element::*, event::*, prelude::dom_css, DomBackend};

use crate::PageMeta;

// write limited CSS
dom_css!(
    // only single class selectors are allowed
    .warn {
        color: orange;
        font-size: 16px;
    }
);

// declare a component
#[component(Backend = DomBackend, Translation = index)]
pub(crate) struct Index {
    // a component should have a template field
    template: template! {
        // the template is XML-like
        <div title="Hello!">
            // strings in the template must be quoted
            "Hello world!"
        </div>
        // use { ... } bindings in the template
        <div tap=@jump() title={ self.hello.to_locale_str() }>
            { &self.hello }
        </div>
        // use classes in `class:xxx` form
        <div class:warn> "WARN" </div>
        // bind event with `@xxx()`
        if !self.r {
            <div tap=@handle_tap()> "Click me!" </div>
        }
    },
    hello: LocaleString,
    r: bool,
}

// implement basic component interfaces
impl Component for Index {
    fn new() -> Self {
        Self {
            template: Default::default(),
            hello: i18n!("Hello world again!").to_locale_string(),
            r: false,
        }
    }
}

impl Index {
    fn jump(_: ComponentRc<Self>, _detail: &mut TapEvent) {
        log::info!("Jump!");
        crate::jump_to("/", "content=def").unwrap();
    }

    // an event handler
    fn handle_tap(this: ComponentRc<Self>, _detail: &mut TapEvent) {
        log::info!("Clicked!");
        this.task(|this| this.r = true);
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct QueryData {
    content: String,
}

#[async_trait]
impl PrerenderableComponent for Index {
    type QueryData = QueryData;
    type PrerenderingData = QueryData;

    async fn prerendering_data(query_data: &Self::QueryData) -> Self::PrerenderingData {
        query_data.clone()
    }

    fn apply_prerendering_data(&mut self, data: Self::PrerenderingData) {
        self.hello = LocaleString::translated(data.content);
    }
}

impl PageMeta for Index {
    fn title(&self) -> &str {
        &self.hello
    }
}
