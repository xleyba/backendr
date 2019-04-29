#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

use actix_web::{App, web, HttpServer};

use colored::*;
use log::{debug};
use log::Level;

mod handlers;
use crate::handlers::index;
use crate::handlers::echo_handler;
use crate::handlers::factorial_iter_handler;
use crate::handlers::factorial_recur_handler;

// Defines the default port
const DEFAULT_PORT: u16          = 9596;

// Defines the workers used by server
const DEFAULT_WORKERS: usize     = 2;

// Config port
#[derive(Deserialize, Debug)]
struct ConfigPort {
    port: u16,
}

// Config Workers
#[derive(Deserialize, Debug)]
struct ConfigWorkers {
    workers: usize,
}

// Displays intro banner
fn intro() {
    println!("{}", "===========================================================".yellow().bold());
    println!("{}", "                    Backendr v 0.1.0".yellow().bold());
    println!("{}", "===========================================================".yellow().bold());
    println!("{}", "   Please use env variables for configuration:".yellow().bold());
    println!("{}", "       BE_PORT=port number".yellow().bold());
    println!("{}", "       BE_WORKERS=workers for server".yellow().bold());
    println!("{}", "       BE_CLIENT_URL=url of called service".yellow().bold());
    println!("{}", "-----------------------------------------------------------");
    println!("Starting configuration......\n");
}

// Configure port through env variables
fn config_port() -> u16 {
    match envy::prefixed("BE_").from_env::<ConfigPort>() {
      Ok(config) => {
          info!("Port set to: {}", config.port);
          config.port
      },
      Err(error) => {
          error!("Error with env var PORT {}", error);
          info!("Port set to {} - default value", DEFAULT_PORT);
          DEFAULT_PORT
      }
   }
}

// Configure workers through env variables
fn config_workers() -> usize {
    match envy::prefixed("BE_").from_env::<ConfigWorkers>() {
      Ok(config) => {
          info!("Workers set to: {}", config.workers);
          config.workers
      },
      Err(error) => {
          error!("Error with env var WORKERS {}", error);
          info!("Workers set to {} - default value", DEFAULT_WORKERS);
          DEFAULT_WORKERS
      }
   }
}



fn main()  -> std::io::Result<()> {

    env_logger::init();
    /*Builder::new()
        .parse(&env::var("BANK_LOG").unwrap_or_default())
        .init();*/

    intro();

    let port = config_port();
    let workers = config_workers();

    println!("{}", "-----------------------------------------------------------");
    println!("Starting server.... Press Ctrl-C to stop it.");

    if log_enabled!(Level::Info) {
        debug!("Starting server");
    }

    HttpServer::new(|| {App::new()
        .service(
            web::resource("/")
                .route(web::get().to(index))
        ) // end service
        .service(
            web::resource("/echo/{message}")
            .route(web::get().to_async(echo_handler))
        ) // end hello service
        .service(
            web::resource("factorialIterative/{number}")
            .route(web::get().to_async(factorial_iter_handler))
        ) // end iter service
        .service(
            web::resource("factorialRecursive/{number}")
            .route(web::get().to_async(factorial_recur_handler))
        ) // end recur service
    })
    .workers(workers)
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    
}