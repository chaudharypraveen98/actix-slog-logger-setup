use actix_web::{web, App, HttpServer};

// IT is used as a logging middleware. We can even use the default logger with actix.
use slog;
use slog::{Logger,o,Drain,info};
use slog_term;
use slog_async;

fn configure_log()->Logger{
    // Formatting the output https://docs.rs/slog-term/2.9.0/slog_term/index.html#
    let decorator = slog_term::TermDecorator::new().build();

    // Drain for outputting https://docs.rs/slog-term/2.9.0/slog_term/index.html#structs
    // fuse is used for panicking if something went wrong. It is necessary to call fuse as the root logger must take a Drain which is error free.
    let console_drain = slog_term::FullFormat::new(decorator).build().fuse();

    // It is used for Synchronization https://docs.rs/slog-term/2.9.0/slog_term/index.html#structs
    let console_drain = slog_async::Async::new(console_drain).build().fuse();
    slog::Logger::root(console_drain,o!("v"=>env!("CARGO_PKG_VERSION")))
}
async fn index(log: web::Data<Logger>) -> &'static str {
    // info is a macro used for building key-value pair https://docs.rs/slog/2.7.0/slog/index.html#macros
    info!(log,
        "Inside Hello World"
    );
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log = configure_log();

    info!(log,
        "Starting the server at http://127.0.0.1:8080/"
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(log.clone()))
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}