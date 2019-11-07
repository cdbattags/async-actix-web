use futures::compat::Stream01CompatExt;
use futures::compat::Compat;
use futures::future::{
    FutureExt,
    TryFutureExt
};
use futures::stream::{
    repeat,
    Repeat,
    StreamExt,
    TryStreamExt
};

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
    let fut = async_stream(req, stream);
    fut.boxed_local().compat()
}

async fn async_message(
    req: HttpRequest,
    stream: web::Payload
) -> Result<HttpResponse> {
    println!("Incoming message request!");

    let mut compat_stream = stream.compat();

    while let Some(data) = compat_stream.try_next().await? {
        println!("{:?}", data);
    }

    Ok(HttpResponse::Ok().content_type("text/html").body("We have liftoff!"))
}

async fn async_stream(
    req: HttpRequest,
    stream: web::Payload
) -> Result<HttpResponse> {
    println!("Incoming stream request!");

    let stream03: Repeat<Result<Bytes, ()>> = repeat(
        Ok(Bytes::from_static(b"Hello, world!"))
    );

    let stream01 = Compat::new(stream03);

    Ok(
        HttpResponse::Ok()
            .content_type("text/html")
            .streaming(stream01)
    )
}

fn main() {
    let port: u32 = 3000;

    println!("Starting HTTP server listening at port {} ...", port);

    let _ = HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header(http::header::CACHE_CONTROL, "no-cache"))
            .wrap(middleware::Logger::default())
            .service(web::resource("/hello-world").to(|| "Hello world!"))
            .service(web::resource("/").route(web::get().to_async(inbound)))
    })
    .bind(format!("0.0.0.0:{}", port))
    .expect(&format!("Unable to bind on port {}", port))
    .run()
    .expect("Failed to start HTTP server");
}
