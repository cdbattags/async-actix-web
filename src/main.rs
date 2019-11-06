use futures::compat::Stream01CompatExt;
use futures::future::{FutureExt, TryFutureExt};
use futures::stream::TryStreamExt;
use futures01::future::Future;
use futures01::stream::Stream;
use futures01::sync::mpsc; // for `try_next`

use actix_web::*;
use bytes::Bytes;
use futures_timer::Delay;
use std::time::Duration;

fn inbound(
    req: HttpRequest,
    stream: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let fut = async_inbound(req, stream);
    fut.boxed_local().compat()
}

async fn async_inbound(
    req: HttpRequest,
    stream: web::Payload
) -> Result<HttpResponse> {
    println!("Incoming request!");

    let mut compat_stream = stream.compat();

    while let Some(data) = compat_stream.try_next().await? {
        println!("{:?}", data);
    }

    Ok(HttpResponse::Ok().content_type("text/html").body("We have liftoff!"))
}

fn main() {
    let port: u32 = 3000;

    println!("Starting HTTP server listening at port {} ...", port);

    let _ = HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header(http::header::CACHE_CONTROL, "no-cache"))
            .wrap(middleware::Logger::default())
            .service(web::resource("/hello-world").to(|| "Hello world!"))
            .service(web::resource("/").route(web::post().to_async(inbound)))
    })
    .bind(format!("0.0.0.0:{}", port))
    .expect(&format!("Unable to bind on port {}", port))
    .run()
    .expect("Failed to start HTTP server");
}
