use maomi::{prelude::*, locale_string::*};
use maomi_dom::{prelude::*, element::*};

use crate::PageMeta;
use crate::components;
use components::page_wrapper::PageWrapper;
use components::utils::code_segment::*;
use components::utils::link::{RawLink, Link};

i18n_group!(index as trans);

stylesheet!(
    use crate::*;

    fn text_style() {
        font_weight = normal;
    }
    const MAIN_TEXT_COLOR: value = rgb(32, 32, 32);
    const DESC_COLOR: value = rgb(64, 64, 64);
    const NOTE_COLOR: value = rgb(128, 128, 128);

    class title_section {
        margin = Px(40) 0;
    }
    class title_icon {
        display = block;
        width = Px(160);
        margin = 0 auto;
    }
    class title {
        text_style();
        text_align = center;
        font_size = Em(2);
        letter_spacing = Em(0.25);
        margin = Em(0.5);
        color = ICON_MAIN;
    }
    class subtitle {
        text_style();
        text_align = center;
        font_size = Em(1.2);
        margin = 0 0 Px(20);
        color = DESC_COLOR;
    }
    class title_code_wrapper {
        display = flex;
        flex_direction = column;
        align_items = center;
    }
    class title_links {
        font_size = Em(0.9);
        text_align = center;
        margin_top = Px(20);
        color = rgb(128, 128, 128);
    }

    class section {
        margin = Px(30) auto;
        max_width = Px(620);
    }
    class section_title {
        text_style();
        font_size = Em(1.3);
        margin = 0 0 Em(0.5);
        color = ICON_MAIN;
    }
    class section_desc {
        text_style();
        color = desc_color;
        margin = 0 0 Em(0.5);
    }
    class section_img_wrapper {
        margin = Px(10) 0;
        padding = Px(10) Px(20);
        border_top = Px(1) solid NOTE_COLOR;
        border_bottom = Px(1) solid NOTE_COLOR;
    }
    class section_img {
        max_width = 100%;
    }
    style width(w: f32) {
        width = Px(w);
    }
    class section_note {
        font_size = Em(0.7);
        color = NOTE_COLOR;
        margin = Em(0.5) 0 0;
    }
    class code_wrapper {}
    class links {
        color = ICON_MAIN;
        font_size = Em(1.1);
    }

    class perf_graph {
        display = flex;
        overflow_x = auto;
        align_items = end;
        padding = Px(10) 0;
    }
    class perf_col {
        flex = 1 0 Em(5);
    }
    class perf_box {
        width = Em(3.3);
        margin = 0 auto;
    }
    fn perf_rect() {
        width = Em(1);
        margin = 0 Em(0.05);
        display = inline_block;
    }
    const GRAPH_TREE_BUILD: value = rgb(255, 200, 154);
    const GRAPH_TREE_UPDATE: value = rgb(244, 180, 176);
    const GRAPH_COMPONENTS: value = rgb(250, 186, 237);
    class perf_tree_build {
        perf_rect();
        background_color = GRAPH_TREE_BUILD;
    }
    class perf_tree_update {
        perf_rect();
        background_color = GRAPH_TREE_UPDATE;
    }
    class perf_components {
        perf_rect();
        background_color = GRAPH_COMPONENTS;
    }
    style perf_height(n: f32) {
        height = Px(n);
    }
    class perf_name {
        white_space = nowrap;
        text_align = center;
    }
    fn perf_graph_note() {
        display = inline_block;
        width = Em(1);
        height = Em(1);
        vertical_align = middle;
        margin_right = Em(0.5);
    }
    class perf_graph_note_tree_build {
        perf_graph_note();
        background_color = GRAPH_TREE_BUILD;
    }
    class perf_graph_note_tree_update {
        perf_graph_note();
        margin_left = Em(1.5);
        background_color = GRAPH_TREE_UPDATE;
    }
    class perf_graph_note_components {
        perf_graph_note();
        margin_left = Em(1.5);
        background_color = GRAPH_COMPONENTS;
    }
);

#[component(Backend = DomBackend, Translation = index)]
pub(crate) struct Index {
    template: template! {
        <PageWrapper>
            <div class:title_section>
                <img src=&{ crate::config::res_path("icon_360.png") } class:title_icon />
                <h1 class:title> "maomi" </h1>
                <p class:subtitle> "Strict and Performant Web Application Programming" </p>
                <div class:title_code_wrapper>
                    <CodeSegment>
                        <CodeLine text=r#"#[component]"# />
                        <CodeLine text=r#"struct HelloWorld {"# />
                        <CodeLine text=r#"    template: template! {"# />
                        <CodeLine text=r#"        "Hello world!""# />
                        <CodeLine text=r#"    }"# />
                        <CodeLine text=r#"}"# />
                    </_>
                </div>
                <div class:title_links>
                    <RawLink url="/maomi/en_US">
                        <span> "English" </span>
                    </RawLink>
                    " | "
                    <RawLink url="/maomi/zh_CN">
                        <span> "简体中文" </span>
                    </RawLink>
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "Programming in Rust" </h2>
                <div class:section_desc>
                    "Write rust code, compile to WebAssembly, and run in browser."
                </div>
                <div class:code_wrapper>
                    <CodeSegment>
                        <CodeComment indent=0 text=&{ trans!(r#"declare a component"#) } />
                        <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                        <CodeLine text=r#"struct HelloWorld {"# />
                        <CodeComment indent=4 text=&{ trans!(r#"write a template"#) } />
                        <CodeLine text=r#"    template: template! {"# />
                        <CodeLine text=r#"        <div class:hello>"# />
                        <CodeLine text=r#"            "Hello world!""# />
                        <CodeLine text=r#"        </div>"# />
                        <CodeLine text=r#"    }"# />
                        <CodeLine text=r#"}"# />
                        <CodeLine text=r#""# />
                        <CodeComment indent=0 text=&{ trans!(r#"implement Component trait"#) } />
                        <CodeLine text=r#"impl Component for HelloWorld {"# />
                        <CodeLine text=r#"    fn new() -> Self {"# />
                        <CodeLine text=r#"        Self {"# />
                        <CodeLine text=r#"            template: Default::default()"# />
                        <CodeLine text=r#"        }"# />
                        <CodeLine text=r#"    }"# />
                        <CodeLine text=r#"}"# />
                        <CodeLine text=r#""# />
                        <CodeComment indent=0 text=&{ trans!(r#"write styles"#) } />
                        <CodeLine text=r#"stylesheet! {"# />
                        <CodeLine text=r#"    class hello {"# />
                        <CodeLine text=r#"        color = rgb(232, 152, 86);"# />
                        <CodeLine text=r#"    }"# />
                        <CodeLine text=r#"}"# />
                    </_>
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "Better Performance" </h2>
                <div class:section_desc>
                    "Maomi has great overall performance and avoids common performance pitfalls. Like rust language itself, there is no worry about performance in maomi application programming."
                </div>
                <div class:perf_graph>
                    for perf in &self.perf_list {
                        <div class:perf_col>
                            <div class:perf_box>
                                <div class:perf_tree_build style:perf_height=&{ perf.tree_build * 1.5 }></div>
                                <div class:perf_tree_update style:perf_height=&{ perf.tree_update * 1.5 }></div>
                                <div class:perf_components style:perf_height=&{ perf.components * 1.5 }></div>
                            </div>
                            <div class:perf_name> { LocaleString::translated(perf.name) } </div>
                        </div>
                    }
                </div>
                <div class:section_note>
                    <div class:perf_graph_note_tree_build></div> "tree build timing"
                    <div class:perf_graph_note_tree_update></div> "tree update timing"
                    <div class:perf_graph_note_components></div> "component abstraction timing"
                </div>
                <div class:section_note>
                    "This DOM manipulation benchmark is based on "
                    <RawLink underline new_page url="https://github.com/krausest/js-framework-benchmark">
                        "js-framework-benchmark"
                    </RawLink>
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "Report Mistakes while Compilation" </h2>
                <div class:section_desc>
                    "Like rust, maomi reports mistakes while compilation. The mistakes include wrong element names, invalid properties, and even wrong style class names."
                </div>
                <div class:section_img_wrapper>
                    <img class:section_img src=&{ crate::config::res_path("ra_error.png") } style:width=322.5 />
                    <img class:section_img src=&{ crate::config::res_path("ra_error_msg.png") } style:width=590 />
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "Work with rust-analyzer in IDE" </h2>
                <div class:section_desc>
                    "With rust-analyzer installed, it is much easier to investigate elements, properties, and even style classes."
                </div>
                <div class:section_img_wrapper>
                    <img class:section_img src=&{ crate::config::res_path("ra_goto.png") } style:width=547 />
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "Data Bindings" </h2>
                <div class:section_desc>
                    "Maomi is based on templates and data bindings."
                </div>
                <div class:code_wrapper>
                    <CodeSegment>
                        <CodeLine text=r#"#[component]"# />
                        <CodeLine text=r#"struct HelloWorld {"# />
                        <CodeLine text=r#"    template: template! {"# />
                        <CodeLine text=r#"        <div class:hello>"# />
                        <CodeComment indent=12 text=&{ trans!("use struct fields in the template") } />
                        <CodeLine text=r#"            { &self.hello }"# />
                        <CodeLine text=r#"        </div>"# />
                        <CodeLine text=r#"    },"# />
                        <CodeLine text=r#"    hello: String,"# />
                        <CodeLine text=r#"}"# />
                    </_>
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "Limited CSS" </h2>
                <div class:section_desc>
                    "Maomi supports a limited stylesheet syntax. It restricts the usage of CSS to make the styling easier to investigate."
                </div>
                <div class:code_wrapper>
                    <CodeSegment>
                        <CodeLine text=r#"stylesheet! {"# />
                        <CodeLine text=r#"    class hello {"# />
                        <CodeLine text=r#"        color = rgb(232, 152, 86);"# />
                        <CodeLine text=r#"        if hover {"# />
                        <CodeLine text=r#"            text_decoration = underline;"# />
                        <CodeLine text=r#"        }"# />
                        <CodeLine text=r#"    }"# />
                        <CodeLine text=r#"}"# />
                    </_>
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "High Performance Server Side Rendering" </h2>
                <div class:section_desc>
                    "Maomi supports server side rendering. It just execute native rust binary in the server to generate HTML output. It is much more performant than using a JavaScript runtime."
                </div>
            </div>
            <div class:section>
                <h2 class:section_title> "Integrated i18n Support" </h2>
                <div class:section_desc>
                    "Maomi supports i18n in the core design. It is easy to compile with TOML-based translation files to generate different version of the application."
                </div>
            </div>
            <div class:section>
                <div class:links>
                    <Link path="/guide"> "Read Guide >" </Link>
                </div>
            </div>
        </PageWrapper>
    },
    perf_list: Vec<Perf>,
}

struct Perf {
    name: &'static str,
    tree_build: f32,
    tree_update: f32,
    components: f32,
}

impl Component for Index {
    fn new() -> Self {
        Self {
            template: Default::default(),
            perf_list: vec![
                Perf { name: "maomi", tree_build: 55.6, tree_update: 47.2, components: 65.3 },
                Perf { name: "react", tree_build: 56.4, tree_update: 211.9, components: 57.1 },
                Perf { name: "vue", tree_build: 49.1, tree_update: 43.6, components: 102.6 },
            ],
        }
    }
}

#[async_trait]
impl PrerenderableComponent for Index {
    type QueryData = ();
    type PrerenderingData = ();

    async fn prerendering_data(_: &Self::QueryData) -> Self::PrerenderingData {
        ()
    }

    fn apply_prerendering_data(&mut self, _: Self::PrerenderingData) {
        ()
    }
}

impl PageMeta for Index {
    fn title(&self) -> LocaleString {
        trans!("maomi - Strict and Performant Web Application Programming").to_locale_string()
    }
}
