use maomi::{prelude::*, locale_string::LocaleStaticStr};
use maomi_dom::{prelude::*, element::*};

use crate::components;
use components::utils::link::*;
use components::page_wrapper::PageWrapper;

pub(crate) mod write_a_component;
pub(crate) mod template_semantics;

i18n_group!(guide as trans);

stylesheet! {
    use crate::*;

    fn text_style() {
        font_weight = normal;
    }

    class wrapper {
        display = flex;
        flex_direction = row_reverse;
        justify_content = center;
        if media (max_width = Px(800)) {
            display = block;
        }
    }

    class side_bar {
        flex = 0 0 auto;
        min_width = Px(200);
        align_self = start;
        border_left = Px(5) solid ICON_SUB;
        padding = Em(1) 0;
        margin = Px(30) Px(40) Px(30) Px(20);
        if media (max_width = Px(800)) {
            margin = Px(20);
        }
    }
    class side_bar_link {
        height = Em(2);
        line_height = Em(2);
        white_space = nowrap;
        padding = 0 Em(1);
    }
    class side_bar_link_active {
        background_color = ICON_SUB;
    }

    class main {
        flex = 1;
        max_width = Px(760);
        margin = Px(20) 0;
    }
    class title {
        text_style();
        font_size = Em(1.5);
        color = ICON_MAIN;
        margin = Em(0.5) 0;
    }
    class content {
        margin = Px(20) 0;
    }
    class footer {
        display = flex;
    }
    class prev {
        flex = 0 0 auto;
        color = ICON_MAIN;
    }
    class footer_center {
        flex = 1;
    }
    class next {
        flex = 0 0 auto;
        color = ICON_MAIN;
    }

    class section {
        margin = Px(30) 0;
    }
    class section_title {
        text_style();
        font_size = Em(1.3);
        margin = Em(0.5) 0;
        color = ICON_MAIN;
    }
    class section_desc {
        text_style();
        color = desc_color;
        margin = Em(0.5) 0;
    }
    class section_img {}
    class code_wrapper {}
}

#[component(Backend = DomBackend)]
pub(crate) struct GuideWrapper {
    template: template! {
        <PageWrapper>
            <div class:wrapper>
                <div class:main>
                    <h1 class:title> { self.chapters[self.cur_chapter_index].title } </h1>
                    <div class:content>
                        <slot />
                    </div>
                    <div class:footer>
                        if self.cur_chapter_index > 0 {
                            if let Some(c) = self.chapters.get(self.cur_chapter_index - 1) {
                                <div class:prev>
                                    <Link path={ c.path }> "< " { c.title } </Link>
                                </div>
                            }
                        }
                        <div class:footer_center />
                        if let Some(c) = self.chapters.get(self.cur_chapter_index + 1) {
                            <div class:next>
                                <Link path={ c.path }> { c.title } " >" </Link>
                            </div>
                        }
                    </div>
                </div>
                <div class:side_bar>
                    for (index, c) in self.chapters.iter().enumerate() {
                        <Link path={ c.path }>
                            <div class:side_bar_link class:side_bar_link_active=&{ index == self.cur_chapter_index }>
                                { c.title }
                            </div>
                        </Link>
                    }
                </div>
            </div>
        </PageWrapper>
    },
    chapters: Vec<Chapter>,
    pub(crate) cur_chapter_path: Prop<String>,
    cur_chapter_index: usize,
}

struct Chapter {
    path: &'static str,
    title: LocaleStaticStr,
}

impl Component for GuideWrapper {
    fn new() -> Self {
        Self {
            template: Default::default(),
            chapters: vec![
                Chapter { path: "/guide", title: trans!("Write a Component") },
                Chapter { path: "/guide/template-semantics", title: trans!("Template Semantics") },
            ],
            cur_chapter_path: Default::default(),
            cur_chapter_index: 0,
        }
    }

    fn before_template_apply(&mut self) {
        self.cur_chapter_index = self.chapters.iter().position(|x| x.path == &*self.cur_chapter_path).unwrap_or_default();
    }
}
