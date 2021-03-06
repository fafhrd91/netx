use std::{env, io};

use futures::future;
use log::info;
use ntex::http::header::HeaderValue;
use ntex::http::{HttpService, Response};
use ntex::server::Server;

#[ntex::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "hello_world=info");
    env_logger::init();

    Server::build()
        .bind("hello-world", "127.0.0.1:8080", || {
            HttpService::build()
                .client_timeout(1000)
                .disconnect_timeout(1000)
                .finish(|_req| {
                    info!("{:?}", _req);
                    let mut res = Response::Ok();
                    res.header("x-head", HeaderValue::from_static("dummy value!"));
                    future::ok::<_, io::Error>(res.body("Hello world!"))
                })
                .tcp()
        })?
        .run()
        .await
}
