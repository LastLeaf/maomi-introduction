use std::cell::RefCell;
use wasm_bindgen::{prelude::*, JsCast};
use maomi::{prelude::*, template::ComponentTemplate, BackendContext, mount_point::DynMountPoint, backend::Backend};
use maomi_dom::prelude::*;

mod components;

macro_rules! route {
    ("/") => { components::index::Index };
}

thread_local! {
    static HISTORY: web_sys::History = {
        let window = web_sys::window().unwrap();
        init_pop_state_listener(&window);
        window.history().unwrap()
    };
    static BACKEND_CONTEXT: RefCell<Option<BackendContext<DomBackend>>> = RefCell::new(None);
    static CURRENT_MOUNT_POINT: RefCell<Option<DynMountPoint<DomBackend>>> = RefCell::new(None);
}

pub(crate) trait PageMeta: PrerenderableComponent + ComponentTemplate<DomBackend> {
    fn title(&self) -> &str;
}

/// Renders a prerenderable component in server side
#[cfg(not(target_arch = "wasm32"))]
pub async fn server_side_rendering(
    req_path: &str,
    query_str: &str,
) -> Result<String, hyper::StatusCode> {
    match req_path {
        "/" => render_component::<route!("/")>(req_path, query_str).await,
        _ => Err(hyper::StatusCode::NOT_FOUND),
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn render_component<T>(
    req_path: &str,
    query_str: &str,
) -> Result<String, hyper::StatusCode>
where
    T: PageMeta,
    T::QueryData: serde::de::DeserializeOwned,
    T::PrerenderingData: serde::Serialize,
{
    let query_data: T::QueryData = match serde_urlencoded::from_str(query_str) {
        Ok(x) => x,
        Err(_) => return Err(hyper::StatusCode::BAD_REQUEST),
    };
    let prerendering_data = BackendContext::<DomBackend>::prerendering_data::<T>(&query_data).await;
    prerender_component_html::<T>(req_path, prerendering_data)
}

#[cfg(not(target_arch = "wasm32"))]
fn prerender_component_html<T>(
    req_path: &str,
    prerendering_data: maomi::PrerenderingData<T>,
) -> Result<String, hyper::StatusCode>
where
    T: PageMeta,
    T::QueryData: serde::de::DeserializeOwned,
    T::PrerenderingData: serde::Serialize,
{
    let dom_backend = DomBackend::prerendering();
    let backend_context = BackendContext::new(dom_backend);
    let prerendering_data_base64 = base64::encode(&bincode::serialize(prerendering_data.get()).unwrap());
    let (_mount_point, title, html_body) = backend_context
        .enter_sync(move |ctx| {
            let mount_point = ctx.prerendering_attach(prerendering_data).unwrap();
            let title = ctx.root_component_with(&mount_point, |c| {
                c.title().to_string()
            });
            let mut html_body = vec![];
            ctx.write_prerendering_html(&mut html_body).unwrap();
            (mount_point, title, html_body)
        })
        .map_err(|_| "Cannot init mount point")
        .unwrap();
    let html = format!(
        include_str!("component_html.tmpl"),
        title = title,
        body = unsafe { std::str::from_utf8_unchecked(&html_body) },
        req_path = req_path,
        data = prerendering_data_base64,
    );
    Ok(html)
}

/// Init the server side rendering result in browser
#[wasm_bindgen]
pub fn server_side_rendering_init(req_path: &str, base64_data: &str) {
    let data = base64::decode(base64_data).unwrap();
    match req_path {
        "/" => server_side_rendering_apply::<route!("/")>(data),
        _ => panic!("Invalid server side rendering path"),
    }
}

fn server_side_rendering_apply<T>(bin: Vec<u8>)
where
    T: PageMeta,
    T::PrerenderingData: serde::de::DeserializeOwned,
{
    let data: T::PrerenderingData = bincode::deserialize(&bin).unwrap();
    let prerendering_data = maomi::PrerenderingData::<T>::new(data);
    let dom_backend = DomBackend::new_prerendered();
    let backend_context = BackendContext::new(dom_backend);
    let mount_point = backend_context
        .enter_sync(move |ctx| {
            let mount_point = ctx.prerendering_attach(prerendering_data).unwrap();
            ctx.apply_prerendered_document_body().unwrap();
            mount_point
        })
        .map_err(|_| "Cannot init mount point")
        .unwrap();
    BACKEND_CONTEXT.with(|x| *x.borrow_mut() = Some(backend_context));
    CURRENT_MOUNT_POINT.with(|x| {
        *x.borrow_mut() = Some(mount_point.into_dyn());
    })
}

/// jump to another page, doing a client side rendering
pub(crate) fn jump_to(
    req_path: &str,
    query_str: &str,
) -> Result<(), String> {
    let url = format!("{}?{}", req_path, query_str);
    HISTORY.with(|history| {
        history.push_state_with_url(&JsValue::NULL, "", Some(&url)).unwrap();
    });
    client_side_rendering(req_path, query_str)
}

fn init_pop_state_listener(window: &web_sys::Window) {
    let closure = Closure::<dyn Fn()>::new(move || {
        let location = web_sys::window().unwrap().document().unwrap().location().unwrap();
        let req_path = location.pathname().unwrap_or("".to_string());
        let search = location.search().unwrap_or("".to_string());
        if let Err(err) = client_side_rendering(&req_path, &search[1..]) {
            log::error!("{}", err);
        }
    }).into_js_value();
    window.add_event_listener_with_callback("popstate", closure.unchecked_ref()).unwrap();
}

fn client_side_rendering(
    req_path: &str,
    query_str: &str,
) -> Result<(), String> {
    match req_path {
        "/" => jump_to_component::<route!("/")>(query_str),
        _ => Err("Invalid rendering path".into()),
    }
}

fn jump_to_component<T>(
    query_str: &str,
) -> Result<(), String>
where
    T: PageMeta,
    T::QueryData: serde::de::DeserializeOwned,
    T::PrerenderingData: serde::Serialize,
{
    let query_data: T::QueryData = match serde_urlencoded::from_str(query_str) {
        Ok(x) => x,
        Err(_) => return Err("bad request".into()),
    };
    DomBackend::async_task(async move {
        let prerendering_data = BackendContext::<DomBackend>::prerendering_data::<T>(&query_data).await;
        BACKEND_CONTEXT.with(|backend_context| {
            let ret = backend_context
                .borrow_mut()
                .as_mut()
                .unwrap()
                .enter_sync(move |ctx| {
                    CURRENT_MOUNT_POINT.with(|x| {
                        if let Some(mut mount_point) = x.borrow_mut().take() {
                            ctx.detach_dyn(&mut mount_point);
                        }
                    });
                    let mount_point = ctx.prerendering_attach(prerendering_data).unwrap();
                    CURRENT_MOUNT_POINT.with(|x| {
                        *x.borrow_mut() = Some(mount_point.into_dyn());
                    });
                });
            if let Err(_) = ret {
                log::error!("Failed jumping to component");
            }
        });
    });
    Ok(())
}

/// init logger on wasm initialization
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Trace).unwrap();
}
