use serde_json::json;
use simple_runtime_demo::choose_starter;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let starter = choose_starter();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "ip": format!("{:?}", _req.headers().get("x-real-ip").unwrap()),
              "region": format!("{:?}, {:?}", _req.headers().get("x-vercel-ip-city").unwrap(),_req.headers().get("x-vercel-ip-country-region").unwrap()),
              "GPS": format!("({:?},{:?})", _req.headers().get("x-vercel-ip-latitude").unwrap(),_req.headers().get("x-vercel-ip-longitude").unwrap()),
            })
            .to_string()
            .into(),
        )?)
}
