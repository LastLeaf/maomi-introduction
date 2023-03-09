use std::cell::RefCell;
use maomi::{prelude::*, template::ComponentTemplate, BackendContext, mount_point::DynMountPoint, locale_string::LocaleString};
use maomi_dom::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, JsCast};

#[cfg(any(feature = "server-side-rendering", target_arch = "wasm32"))]
mod components;

#[cfg(any(feature = "server-side-rendering", target_arch = "wasm32"))]
type DefaultComponent = components::not_found::NotFound;

#[cfg(any(feature = "server-side-rendering", target_arch = "wasm32"))]
macro_rules! for_each_route {
    ($mac:ident) => {
        $mac!("/", components::index::Index);
        $mac!("/guide", components::guide::write_a_component::Content);
        $mac!("/guide/template-nodes", components::guide::template_nodes::Content);
        $mac!("/guide/template-branches", components::guide::template_branches::Content);
    };
}

thread_local! {
    #[cfg(target_arch = "wasm32")]
    static HISTORY: web_sys::History = {
        let window = web_sys::window().unwrap();
        init_pop_state_listener(&window);
        window.history().unwrap()
    };
    static BACKEND_CONTEXT: RefCell<Option<BackendContext<DomBackend>>> = RefCell::new(None);
    static CURRENT_MOUNT_POINT: RefCell<Option<DynMountPoint<DomBackend>>> = RefCell::new(None);
}

pub(crate) trait PageMeta: PrerenderableComponent + ComponentTemplate<DomBackend> {
    fn title(&self) -> LocaleString;
}

/// Renders a prerenderable component in server side
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "server-side-rendering")]
pub async fn server_side_rendering(
    req_path: &str,
    query_str: &str,
) -> Result<String, hyper::StatusCode> {
    macro_rules! route {
        ($p:expr, $c:ty) => {
            if (req_path == $p) {
                return render_component::<$c>(req_path, query_str).await;
            }
        };
    }
    for_each_route!(route);
    return render_component::<DefaultComponent>(req_path, query_str).await;
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "server-side-rendering")]
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
#[cfg(feature = "server-side-rendering")]
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
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn html_init(req_path: &str, data: &str) {
    #[cfg(feature = "server-side-rendering")]
    {
        let data = base64::decode(data).unwrap();
        macro_rules! route {
            ($p:expr, $c:ty) => {
                if (req_path == $p) {
                    return server_side_rendering_apply::<$c>(data);
                }
            };
        }
        for_each_route!(route);
        return server_side_rendering_apply::<DefaultComponent>(data);
    }
    #[cfg(not(feature = "server-side-rendering"))]
    {
        let dom_backend = DomBackend::new_with_document_body().unwrap();
        let backend_context = BackendContext::new(dom_backend);
        BACKEND_CONTEXT.with(|x| *x.borrow_mut() = Some(backend_context));
        client_side_rendering(req_path, data).unwrap();
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg(feature = "server-side-rendering")]
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
#[allow(dead_code)]
pub(crate) fn jump_to(
    _req_path: &str,
    _query_str: &str,
) {
    #[cfg(target_arch = "wasm32")]
    {
        let url = if _query_str.len() > 0 {
            format!("{}?{}", _req_path, _query_str)
        } else {
            format!("{}", _req_path)
        };
        HISTORY.with(|history| {
            history.push_state_with_url(&JsValue::NULL, "", Some(&url)).unwrap();
        });
        web_sys::window().unwrap()
            .document().unwrap()
            .document_element().unwrap()
            .set_scroll_top(0);
        if let Err(err) = client_side_rendering(_req_path, _query_str) {
            log::error!("{}", err);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn init_pop_state_listener(window: &web_sys::Window) {
    let closure = Closure::<dyn Fn()>::new(move || {
        let location = web_sys::window().unwrap().document().unwrap().location().unwrap();
        let req_path = location.pathname().unwrap_or("".to_string());
        let search = location.search();
        let search = match search.as_ref() {
            Ok(x) if x.len() == 0 => "?",
            Err(_) => "?",
            Ok(x) => x,
        };
        log::warn!("!!! {:?} {:?}", req_path, search);
        if let Err(err) = client_side_rendering(&req_path, &search[1..]) {
            log::error!("{}", err);
        }
    }).into_js_value();
    window.add_event_listener_with_callback("popstate", closure.unchecked_ref()).unwrap();
}

#[cfg(target_arch = "wasm32")]
fn client_side_rendering(
    req_path: &str,
    query_str: &str,
) -> Result<(), String> {
    macro_rules! route {
        ($p:expr, $c:ty) => {
            if (req_path == $p) {
                return jump_to_component::<$c>(query_str);
            }
        };
    }
    for_each_route!(route);
    return jump_to_component::<DefaultComponent>(query_str);
}

#[cfg(target_arch = "wasm32")]
fn jump_to_component<T>(
    query_str: &str,
) -> Result<(), String>
where
    T: PageMeta,
    T::QueryData: serde::de::DeserializeOwned,
    T::PrerenderingData: serde::Serialize,
{
    use maomi::backend::Backend;
    let query_data: T::QueryData = match serde_urlencoded::from_str(query_str) {
        Ok(x) => x,
        Err(_) => return Err("bad request".into()),
    };
    maomi_dom::DomBackend::async_task(async move {
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
                    mount_point.root_component().rc().task_with(|this, _| {
                        let title = this.title();
                        web_sys::window().unwrap().document().unwrap().set_title(&title);
                    });
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
