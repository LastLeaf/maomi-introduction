use maomi::{prelude::*, locale_string::LocaleStaticStr};
use maomi_dom::{prelude::*, element::*};

use crate::components;
use components::utils::link::*;
use components::page_wrapper::PageWrapper;

pub(crate) mod write_a_component;
pub(crate) mod template_nodes;
pub(crate) mod template_branches;
pub(crate) mod runtime_performance_tips;
pub(crate) mod compilation_performance_tips;

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
        padding = Em(0.5) 0;
        margin = Px(30) Px(40) Px(30) Px(20);
        if media (max_width = Px(800)) {
            margin = Px(20);
        }
    }
    class side_bar_section {
        padding = Em(0.25) 0;
    }
    class side_bar_section_title {
        height = Em(2);
        line_height = Em(2);
        white_space = nowrap;
        padding = 0 Em(1);
        color = TEXT_SUB;
    }
    class side_bar_link {
        height = Em(2);
        line_height = Em(2);
        white_space = nowrap;
        padding = 0 Em(1) 0 Em(2);
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
                    <h1 class:title> { self.get_chapter(&self.cur_chapter).title } </h1>
                    <div class:content>
                        <slot />
                    </div>
                    <div class:footer>
                        if let Some(c) = self.prev_chapter.map(|x| self.get_chapter(x)) {
                            <div class:prev>
                                <Link path={ c.path }> "< " { c.title } </Link>
                            </div>
                        }
                        <div class:footer_center />
                        if let Some(c) = self.next_chapter.map(|x| self.get_chapter(x)) {
                            <div class:next>
                                <Link path={ c.path }> { c.title } " >" </Link>
                            </div>
                        }
                    </div>
                </div>
                <div class:side_bar>
                    for cg in self.chapters.iter() {
                        <div class:side_bar_section>
                            <div class:side_bar_section_title> { cg.title } </div>
                            for c in cg.chapters.iter() {
                                <Link path={ c.path }>
                                    <div class:side_bar_link class:side_bar_link_active=&{ c.path == &*self.cur_chapter }>
                                        { c.title }
                                    </div>
                                </Link>
                            }
                        </div>
                    }
                </div>
            </div>
        </PageWrapper>
    },
    chapters: Vec<ChapterGroup>,
    pub(crate) cur_chapter: Prop<String>,
    prev_chapter: Option<&'static str>,
    next_chapter: Option<&'static str>,
}

struct ChapterGroup {
    title: LocaleStaticStr,
    chapters: Vec<Chapter>,
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
                ChapterGroup {
                    title: trans!("Basics"),
                    chapters: vec![
                        Chapter { path: "/guide", title: trans!("Write a Component") },
                    ],
                },
                ChapterGroup {
                    title: trans!("Template Semantics"),
                    chapters: vec![
                        Chapter { path: "/guide/template-nodes", title: trans!("Template Nodes") },
                        Chapter { path: "/guide/template-branches", title: trans!("Template Branches") },
                    ],
                },
                ChapterGroup {
                    title: trans!("Performance Tips"),
                    chapters: vec![
                        Chapter { path: "/guide/runtime-performance-tips", title: trans!("Runtime Performance Tips") },
                        Chapter { path: "/guide/compilation-performance-tips", title: trans!("Compilation Performance Tips") },
                    ],
                },
            ],
            cur_chapter: Default::default(),
            prev_chapter: None,
            next_chapter: None,
        }
    }

    fn before_template_apply(&mut self) {
        self.prev_chapter = None;
        self.next_chapter = None;
        let mut found = false;
        for cg in self.chapters.iter() {
            for c in cg.chapters.iter() {
                if found {
                    self.next_chapter = Some(c.path);
                    break;
                }
                if c.path == self.cur_chapter.as_str() {
                    found = true;
                    continue;
                }
                self.prev_chapter = Some(c.path);
            }
        }
    }
}

impl GuideWrapper {
    fn get_chapter(&self, path: &str) -> &Chapter {
        for cg in self.chapters.iter() {
            for c in cg.chapters.iter() {
                if c.path == path {
                    return c;
                }
            }
        }
        unreachable!()
    }
}
