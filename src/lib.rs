use worker::*;
mod chat;
mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    log_request(&req);

    utils::set_panic_hook();

    let router = Router::new();

    router
        .get_async("/rooms/:name/*", get_room)
        .run(req, env)
        .await
}

async fn get_room<D>(req: Request, ctx: RouteContext<D>) -> Result<Response> {
    let name_param = ctx.param("name");
    if let Some(name) = name_param {
        if name.len() > 32 {
            return Response::error("name too long", 401);
        }

        let namespace = ctx.durable_object("rooms")?;
        let id = namespace.id_from_name(name)?;

        let stub = id.get_stub()?;

        let url = req.url()?;
        let prefix = format!("/rooms/{}/", name);

        if let Some(stripped_path) = url.path().strip_prefix(&prefix) {
            return stub.fetch_with_str(stripped_path).await;
        }
    }

    Response::error("unexpected", 500)
}
