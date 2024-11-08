extern crate actix_files;
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();
    info!("Starting up!");

    let (address, directory): (String, String) = {
        use clap::Arg;

        let app = command!()
            .arg(
                Arg::new("address")
                    .long("address")
                    .short('a')
                    .num_args(1)
                    .default_value("localhost:8080")
                    .help("Address to bind to"),
            )
            .arg(
                Arg::new("directory")
                    .num_args(1)
                    .long("directory")
                    .short('d')
                    .default_value("."),
            );

        let matches = app.get_matches();

        let address: String = matches.get_one::<String>("address").unwrap().to_string();
        let directory: String = matches.get_one::<String>("directory").unwrap().to_string();

        (address, directory)
    };
    info!("serving files in {} on {}", directory, address);

    {
        use actix_web::{App, HttpServer};

        HttpServer::new(move || {
            App::new()
                .wrap(actix_web::middleware::Logger::default())
                .service(
                    actix_files::Files::new("/", directory.to_owned())
                        .show_files_listing()
                        .prefer_utf8(true),
                )
        })
        .bind(address)?
        .run()
        .await
    }
}
