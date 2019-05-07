#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate validator_derive;
extern crate validator;
extern crate r2d2;
extern crate r2d2_sqlite;

use actix_web::{App, guard, HttpResponse, web, HttpServer};

use r2d2_sqlite::SqliteConnectionManager;

use colored::*;
use log::{debug};
use log::Level;

mod handlers;
use crate::handlers::index;
use crate::handlers::echo_handler;
use crate::handlers::customer_accounts_handler;
use crate::handlers::customer_account_handler;
use crate::handlers::customer_account_detail_handler;
use crate::handlers::customer_account_movements_handler;
use crate::handlers::customer_account_movements_top_handler;
use crate::handlers::customer_account_movements_balance_handler;

// Defines the default port
const DEFAULT_PORT: u16          = 9596;

// Defines the workers used by server
const DEFAULT_WORKERS: usize     = 2;

// Defines the DB Path 
const DEFAULT_DB_PATH: &str      = "./DB/zerodb.db";

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

// Config DB Path
#[derive(Deserialize, Debug)]
struct ConfigDbPath {
    db_path: String,
}

// Displays intro banner
fn intro() {
    println!("{}", "===========================================================".yellow().bold());
    println!("{}", "                    Backendr v 0.1.0".yellow().bold());
    println!("{}", "===========================================================".yellow().bold());
    println!("{}", "   Please use env variables for configuration:".yellow().bold());
    println!("{}", "       BE_PORT=port number".yellow().bold());
    println!("{}", "       BE_WORKERS=workers for server".yellow().bold());
    println!("{}", "       BE_DB_PATH=path/name of DB".yellow().bold());
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

// Configure workers through env variables
fn config_db_path() -> String {
    match envy::prefixed("BE_").from_env::<ConfigDbPath>() {
      Ok(config) => {
          info!("DB Path set to: {}", config.db_path);
          config.db_path
      },
      Err(error) => {
          error!("Error with env var DB_PATH {}", error);
          info!("Workers set to {} - default value", DEFAULT_DB_PATH);
          DEFAULT_DB_PATH.to_string()
      }
   }
}


// ------------------------------------------
	// /customer/accounts					- 
	// /customer/account					- 
	// /customer/account/detail				- 
	// /customer/account/movements			- 
	// /customer/account/movements/top		- 
	// /customer/account/movements/balance	- 
	// ------------------------------------------

fn main()  -> std::io::Result<()> {

    env_logger::init();
    /*Builder::new()
        .parse(&env::var("BANK_LOG").unwrap_or_default())
        .init();*/

    intro();

    let port = config_port();
    let workers = config_workers();
    let db_path = config_db_path();

    /*let con = Connection::open(db_path);
    let c = match con {
        Ok(c) => c,
        Err(error) => {
            panic!("Error connecting to db {}", error)
        },
    };*/

    let manager = SqliteConnectionManager::file(db_path);
    let pool = r2d2::Pool::new(manager).unwrap();   

    println!("{}", "-----------------------------------------------------------");
    println!("Starting server.... Press Ctrl-C to stop it.");

    if log_enabled!(Level::Info) {
        debug!("Starting server");
    }

    HttpServer::new(
        move || {
            App::new()
        .data(pool.clone(),) // <- store db pool in app state
        .service(
            web::resource("/")
                .route(web::get().to(index))
        ) // end service
        .service(
            web::resource("/hello")
            .route(web::get().to(echo_handler))
        ) // end hello service
        .service(
            web::resource("/customer/accounts")
            .route(web::get().to_async(customer_accounts_handler))
        ) // end customer accounts
        .service(
            web::resource("/customer/account")
            .route(web::get().to_async(customer_account_handler))
        ) // end customer account
        .service(
            web::resource("/customer/account/detail")
            .route(web::get().to_async(customer_account_detail_handler))
        ) // end customer account details
        .service(
            web::resource("/customer/account/movements")
            .route(web::get().to_async(customer_account_movements_handler))
        ) // end customer account movements 
        .service(
            web::resource("/customer/account/movements/top")
            .route(web::get().to_async(customer_account_movements_top_handler))
        ) // end customer account movements top   
        .service(
            web::resource("/customer/account/movements/balance")
            .route(web::get().to_async(customer_account_movements_balance_handler))
        ) // end customer account movements balance              
        .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get()
                      //.to(p404)
                      .to(|| {
                          println!("Response for wrong url");
                          HttpResponse::NotFound()
                      }))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(|| HttpResponse::MethodNotAllowed()),
                    ),
            )
    })
    .workers(workers)
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    
}