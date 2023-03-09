use maomi::{prelude::*, locale_string::*};
use maomi_dom::{prelude::*, element::*};

use crate::{PageMeta, components::utils::{code_segment::*, link::RawLink}};
use super::{GuideWrapper, section, section_title, section_desc};

i18n_group!(guide as trans);

stylesheet!(
    use crate::*;
);

#[component(Backend = DomBackend, Translation = guide)]
pub(crate) struct Content {
    template: template! {
        <GuideWrapper cur_chapter="/guide">
            <div class:section>
                <h2 class:section_title> "Add dependencies in Cargo.toml" </h2>
                <p class:section_desc>
                    "To use maomi, rust 1.65+ is required. Then add maomi as cargo dependencies. In Cargo.toml:"
                </p>
                <CodeSegment>
                    <CodeLine text=r#"[package]"# />
                    <CodeLine text=r#"name = "my-website""# />
                    <CodeLine text=r#"version = "0.1.0""# />
                    <CodeLine text=r#"edition = "2021""# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"[lib]"# />
                    <CodeLine text=r#"crate-type = ["cdylib", "rlib"]"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"[dependencies]"# />
                    <CodeLine text=r#"maomi = "0.4""# />
                    <CodeLine text=r#"maomi-dom = "0.4""# />
                </_>
                <p class:section_desc>
                    "In real world, some other crates are also needed:"
                </p>
                <CodeSegment>
                    <CodeLine text=r#"[dependencies]"# />
                    <CodeLine text=r#"maomi = "0.4""# />
                    <CodeLine text=r#"maomi-dom = "0.4""# />
                    <CodeLine em=&{Emphasize} text=r#"wasm-bindgen = "0.2""# />
                    <CodeLine em=&{Emphasize} text=r#"log = "0.4""# />
                    <CodeLine em=&{Emphasize} text=r#"console_log = "0.2""# />
                    <CodeLine em=&{Emphasize} text=r#"console_error_panic_hook = "0.1""# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "#[component] struct" </h2>
                <p class:section_desc>
                    "A page is composed by components. To generate a page with maomi, write a component first."
                </p>
                <p class:section_desc>
                    r##"A component is a struct with "#[component]" attribute:"##
                </p>
                <CodeSegment>
                    <CodeLine text=r#"use wasm_bindgen::prelude::*;"# />
                    <CodeLine text=r#"use maomi::prelude::*;"# />
                    <CodeLine text=r#"use maomi_dom::{prelude::*, element::*};"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine text=r#"        <div> "My Website" </div>"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"The struct must contain a "template!" field. It contains the template of this component."#
                </p>
            </div>

            <div class:section>
                <h2 class:section_title> "Implement Component trait" </h2>
                <p class:section_desc>
                    r#"The "Component" trait should be implemented for the component struct:"#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"impl Component for MyWebsite {"# />
                    <CodeLine text=r#"    fn new() -> Self {"# />
                    <CodeLine text=r#"        Self {"# />
                    <CodeLine text=r#"            template: Default::default(),"# />
                    <CodeLine text=r#"        }"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Render the Component into page body" </h2>
                <p class:section_desc>
                    r#"Finally, this component can be put into the page body."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[wasm_bindgen(start)]"# />
                    <CodeLine text=r#"pub fn wasm_main() {"# />
                    <CodeComment indent=4 text=&{ trans!(r#"enable console log"#) } />
                    <CodeLine text=r#"    console_error_panic_hook::set_once();"# />
                    <CodeLine text=r#"    console_log::init_with_level(log::Level::Trace).unwrap();"# />
                    <CodeLine text=r#""# />
                    <CodeComment indent=4 text=&{ trans!(r#"initialize a backend context in HTML <body>"#) } />
                    <CodeLine text=r#"    let dom_backend = DomBackend::new_with_document_body().unwrap();"# />
                    <CodeLine text=r#"    let backend_context = maomi::BackendContext::new(dom_backend);"# />
                    <CodeLine text=r#""# />
                    <CodeComment indent=4 text=&{ trans!(r#"create a mount point for the component"#) } />
                    <CodeLine text=r#"    let mount_point = backend_context"# />
                    <CodeLine text=r#"        .enter_sync(move |ctx| {"# />
                    <CodeLine text=r#"            ctx.attach(|_: &mut MyWebsite| {})"# />
                    <CodeLine text=r#"        })"# />
                    <CodeLine text=r#"        .map_err(|_| "Cannot init mount point")"# />
                    <CodeLine text=r#"        .unwrap();"# />
                    <CodeLine text=r#""# />
                    <CodeComment indent=4 text=&{ trans!(r#"leak the mount point and the backend context to keep working forever"#) } />
                    <CodeLine text=r#"    std::mem::forget(mount_point);"# />
                    <CodeLine text=r#"    std::mem::forget(backend_context);"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Compile to WebAssembly" </h2>
                <p class:section_desc>
                    r#"Compile the code with "#
                    <RawLink underline url="https://rustwasm.github.io/wasm-pack/"> "wasm-pack" </_>
                </p>
                <CodeSegment>
                    <CodeLine text=r#"wasm-pack build --target no-modules"# />
                </_>
                <p class:section_desc>
                    r#"The generated code is under the "pkg" directory."#
                </p>
            </div>

            <div class:section>
                <h2 class:section_title> "Write an HTML wrapper" </h2>
                <p class:section_desc>
                    r#"Write an HTML file with empty <body>."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"<!DOCTYPE html>"# />
                    <CodeLine text=r#"<html>"# />
                    <CodeLine text=r#"    <head>"# />
                    <CodeLine text=r#"        <meta charset="UTF-8">"# />
                    <CodeLine text=r#"        <meta name="viewport" content="width=device-width,initial-scale=1">"# />
                    <CodeLine text=r#"    </head>"# />
                    <CodeLine text=r#"    <body></body>"# />
                    <CodeLine text=r#"    <script src="pkg/my_website.js"></script>"# />
                    <CodeLine text=r#"    <script>"# />
                    <CodeLine text=r#"        wasm_bindgen('pkg/my_website_bg.wasm')"# />
                    <CodeLine text=r#"    </script>"# />
                    <CodeLine text=r#"</html>"# />
                </_>
                <p class:section_desc>
                    r#"Then serve it in an HTTP server and see the result!"#
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
        trans!("{} - {}", trans!("maomi Guide"), trans!("Write a Component")).to_locale_string()
    }
}
