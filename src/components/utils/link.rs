use maomi::prelude::*;
use maomi_dom::{prelude::*, element::*, event::*};

dom_css! {
    .inner {
        text-decoration: none;
        color: inherit;
    }
    .underline {
        text-decoration: underline;
    }
}

#[component(Backend = DomBackend)]
pub(crate) struct Link {
    template: template! {
        <a class:inner href=&{ format!("{}?{}", self.path, self.query) } tap=@tap() click=@click()>
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

#[component(Backend = DomBackend)]
pub(crate) struct RawLink {
    template: template! {
        <a
            class:inner
            class:underline=&{ self.underline }
            href=&{ *self.url }
            target={ if *self.new_page { "_blank" } else { "" } }
        >
            <slot />
        </a>
    },
    pub url: Prop<String>,
    pub new_page: Prop<bool>,
    pub underline: Prop<bool>,
}

// implement basic component interfaces
impl Component for RawLink {
    fn new() -> Self {
        Self {
            template: Default::default(),
            url: Default::default(),
            new_page: Default::default(),
            underline: Default::default(),
        }
    }
}
