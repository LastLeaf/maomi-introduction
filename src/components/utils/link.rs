use maomi::prelude::*;
use maomi_dom::{prelude::*, element::*, event::*};

dom_css! {
    // empty
}

#[component(Backend = DomBackend)]
pub(crate) struct Link {
    template: template! {
        <a href=&{ format!("{}?{}", self.path, self.query) } tap=@tap() click=@click()>
            <slot />
        </a>
    },
    pub path: Prop<String>,
    pub query: Prop<String>,
}

// implement basic component interfaces
impl Component for Link {
    fn new() -> Self {
        Self {
            template: Default::default(),
            path: Default::default(),
            query: Default::default(),
        }
    }
}

impl Link {
    fn tap(this: ComponentRc<Self>, _: &mut TapEvent) {
        this.task_with(|this, _| {
            crate::jump_to(&this.path, &this.query);
        });
    }

    fn click(_: ComponentRc<Self>, detail: &mut MouseEvent) {
        detail.prevent_default();
    }
}
