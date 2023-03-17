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
        <GuideWrapper cur_chapter="/guide/server-side-rendering">
            <div class:section>
                <h2 class:section_title> "How server side rendering works" </h2>
                <p class:section_desc>
                    r#"Normally, maomi generates HTML elements in browsers. At startup, it takes an empty DOM node, a.k.a. a mount-point, and then repeatedly insert or modify other nodes."#
                </p>
                <p class:section_desc>
                    r#"The server side rendering can generate static HTML segments in the server. The HTML segment can be inserted into the mount-point before startup. Then maomi will reuse the generated HTML nodes at startup."#
                </p>
                <p class:section_desc>
                    r#"This allows static contents embed in HTML directly, and be understood by search engines better."#
                </p>
                <p class:section_desc>
                    r#"Because maomi is in rust, the component code can be compiled to native binary which can be used to generate HTML segments. It means the components are compiled twice: one is to the native binary which is used to generate the static HTML, the other is to the WebAssembly binary which is used in browser runtime."#
                </p>
                <p class:section_desc>
                    r#"To enable server side rendering, some features need to be specified in Cargo.toml. The "prerendering" feature is to enable the server side support, and the "prerendering-apply" feature is to enable the reuse of the server side generated nodes."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"[dependencies]"# />
                    <CodeLine text=r#"maomi = { version = "0.4", features = ["prerendering", "prerendering-apply"] }"# />
                    <CodeLine text=r#"maomi-dom = { version = "0.4", features = ["prerendering", "prerendering-apply"] }"# />
                    <CodeLine text=r#"async-trait = "0.1""# />
                    <CodeLine text=r#"serde = { version = "1", features = ["derive"] }"# />
                    <CodeLine text=r#"bincode = "1.3""# />
                </_>
                <p class:section_desc>
                    r#"Note that these features slightly increase the runtime overhead. Do not enable them if server side rendering is not needed."#
                </p>
            </div>

            <div class:section>
                <h2 class:section_title> "Prerenderable Components" </h2>
                <p class:section_desc>
                    r#"To be rendered in server side, the component must implement "PrerenderableComponent". This trait has two associated types and two functions."#
                </p>
                <p class:section_desc>
                    r#"The "QueryData" type refers to the query, e.g. the URL params, the POST data, or some other related data. For static contents, it can simply be "()"."#
                </p>
                <p class:section_desc>
                    r#"The "PrerenderingData" type is converted from the "QueryData", containing some useful parts of the query for the component startup. It should be serializable and transferable from server side to the client side."#
                </p>
                <p class:section_desc>
                    r#"When doing server side rendering, firstly, the "prerendering_data" function is called in server side to convert "QueryData" to "PrerenderingData". (It may also be called in client side when doing client side rendering.)"#
                </p>
                <p class:section_desc>
                    r#"Secondly, the component starts in the server side, and the "apply_prerendering_data" is called. This function can modify component fields according to the "PrerenderingData". The HTML segment can be generated when the component created."#
                </p>
                <p class:section_desc>
                    r#"Thirdly, the generated HTML should be embed into the mount-point, and the "PrerenderingData" should also be transferred to the client side."#
                </p>
                <p class:section_desc>
                    r#"Fourthly, the component starts in the client side, and the "apply_prerendering_data" is also called. This function must do the same thing as it did in the server side. Thus this component can reuse the generated HTML nodes."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine text=r#"        <div> "My Website" </div>"# />
                    <CodeLine text=r#"    },"# />
                    <CodeLine text=r#"    content: String,"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"impl Component for MyWebsite {"# />
                    <CodeLine text=r#"    fn new() -> Self {"# />
                    <CodeLine text=r#"        Self {"# />
                    <CodeLine text=r#"            template: Default::default(),"# />
                    <CodeLine text=r#"            content: String::new(),"# />
                    <CodeLine text=r#"        }"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"#[async_trait::async_trait]"# />
                    <CodeLine text=r#"impl PrerenderableComponent for MyWebsite {"# />
                    <CodeLine text=r#"    type QueryData = ();"# />
                    <CodeLine text=r#"    type PrerenderingData = ();"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"    async fn prerendering_data(_: &Self::QueryData) -> Self::PrerenderingData {"# />
                    <CodeLine text=r#"        ()"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"    fn apply_prerendering_data(&mut self, _: Self::PrerenderingData) {"# />
                    <CodeLine text=r#"        self.content = "Prerendering String".to_string();"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Note that these features slightly increase the runtime overhead. Do not enable them if server side rendering is not needed."#
                </p>
            </div>

            <div class:section>
                <h2 class:section_title> "Generate HTML in server side" </h2>
                <p class:section_desc>
                    r#"To generate HTML in server side, a special kind of backend context is used."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"use maomi::template::ComponentTemplate;"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"async fn prerendering<C: PrerenderableComponent + ComponentTemplate<DomBackend>>("# />
                    <CodeLine text=r#"    query_data: &C::QueryData,"# />
                    <CodeLine text=r#") -> (Vec<u8>, Vec<u8>)"# />
                    <CodeLine text=r#"where"# />
                    <CodeLine text=r#"    C::PrerenderingData: serde::Serialize,"# />
                    <CodeLine text=r#"{"# />
                    <CodeComment indent=4 text=&{ trans!(r#"generate the `PrerenderingData`"#) } />
                    <CodeLine text=r#"    let prerendering_data = maomi::BackendContext::<DomBackend>::prerendering_data::<C>(&query_data).await;"# />
                    <CodeLine text=r#""# />
                    <CodeComment indent=4 text=&{ trans!(r#"serialize the `PrerenderingData` to transfer to the client side"#) } />
                    <CodeLine text=r#"    let prerendering_data_bin = bincode::serialize(prerendering_data.get()).unwrap();"# />
                    <CodeLine text=r#""# />
                    <CodeComment indent=4 text=&{ trans!(r#"initialize a backend context for HTML generation"#) } />
                    <CodeLine text=r#"    let prerendering_dom_backend = DomBackend::prerendering();"# />
                    <CodeLine text=r#"    let backend_context = maomi::BackendContext::new(prerendering_dom_backend);"# />
                    <CodeLine text=r#""# />
                    <CodeComment indent=4 text=&{ trans!(r#"generate HTML segment"#) } />
                    <CodeLine text=r#"    let (_mount_point, html_buffer) = backend_context"# />
                    <CodeLine text=r#"        .enter_sync(move |ctx| {"# />
                    <CodeLine text=r#"            let mount_point = ctx.prerendering_attach(prerendering_data).unwrap();"# />
                    <CodeLine text=r#"            let mut html_buffer = vec![];"# />
                    <CodeLine text=r#"            ctx.write_prerendering_html(&mut html_buffer).unwrap();"# />
                    <CodeLine text=r#"            (mount_point, html_buffer)"# />
                    <CodeLine text=r#"        })"# />
                    <CodeLine text=r#"        .map_err(|_| "Cannot init mount point")"# />
                    <CodeLine text=r#"        .unwrap();"# />
                    <CodeLine text=r#""# />
                    <CodeComment indent=4 text=&{ trans!(r#"returns the HTML and the `PrerenderingData`"#) } />
                    <CodeComment indent=4 text=&{ trans!(r#"the HTML should finally be placed in the HTML <body />"#) } />
                    <CodeComment indent=4 text=&{ trans!(r#"the `PrerenderingData` can be base64 embed to HTML or transfer in other forms"#) } />
                    <CodeLine text=r#"    (html_buffer, prerendering_data_bin)"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Note that these features slightly increase the runtime overhead. Do not enable them if server side rendering is not needed."#
                </p>
            </div>

            <div class:section>
                <h2 class:section_title> "Reuse the generated HTML in client side" </h2>
                <p class:section_desc>
                    r#"To reuse the generated HTML, the backend context should be initialized in with "PrerenderingData"."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"use maomi::template::ComponentTemplate;"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"fn prerendering_apply<C: PrerenderableComponent + ComponentTemplate<DomBackend>>("# />
                    <CodeLine text=r#"    prerendering_data_bin: Vec<u8>,"# />
                    <CodeLine text=r#")"# />
                    <CodeLine text=r#"where"# />
                    <CodeLine text=r#"    for<'de> C::PrerenderingData: serde::Deserialize<'de>,"# />
                    <CodeLine text=r#"{"# />
                    <CodeComment indent=4 text=&{ trans!(r#"deserialize the `PrerenderingData`"#) } />
                    <CodeLine text=r#"    let data = bincode::deserialize(&prerendering_data_bin).unwrap();"# />
                    <CodeLine text=r#""# />
                    <CodeComment indent=4 text=&{ trans!(r#"construct the `PrerenderingData`"#) } />
                    <CodeLine text=r#"    let prerendering_data = maomi::PrerenderingData::<C>::new(data);"# />
                    <CodeLine text=r#""# />
                    <CodeComment indent=4 text=&{ trans!(r#"initialize a backend context for reuse generated HTML"#) } />
                    <CodeLine text=r#"    let dom_backend = DomBackend::new_prerendered();"# />
                    <CodeLine text=r#"    let backend_context = maomi::BackendContext::new(dom_backend);"# />
                    <CodeLine text=r#""# />
                    <CodeComment indent=4 text=&{ trans!(r#"create the mount point"#) } />
                    <CodeLine text=r#"    let mount_point = backend_context"# />
                    <CodeLine text=r#"        .enter_sync(move |ctx| {"# />
                    <CodeLine text=r#"            let mount_point = ctx.prerendering_attach(prerendering_data).unwrap();"# />
                    <CodeLine text=r#"            ctx.apply_prerendered_document_body().unwrap();"# />
                    <CodeLine text=r#"            mount_point"# />
                    <CodeLine text=r#"        })"# />
                    <CodeLine text=r#"        .map_err(|_| "Cannot init mount point")"# />
                    <CodeLine text=r#"        .unwrap();"# />
                    <CodeLine text=r#""# />
                    <CodeComment indent=4 text=&{ trans!(r#"leak the mount point and the backend context to keep working forever"#) } />
                    <CodeLine text=r#"    std::mem::forget(mount_point);"# />
                    <CodeLine text=r#"    std::mem::forget(backend_context);"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Note that these features slightly increase the runtime overhead. Do not enable them if server side rendering is not needed."#
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
        trans!("{} - {}", trans!("maomi Guide"), trans!("Server Side Rendering")).to_locale_string()
    }
}
