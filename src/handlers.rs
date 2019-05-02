use actix_web::{Result, Error, web, HttpResponse};
use futures::Future;

use uuid::Uuid;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{NO_PARAMS};

mod models;
use crate::handlers::models::CustomerAccount;
use crate::handlers::models::CustomerAccounts;

// Handle index route
pub fn index() -> &'static str {
    "Hello world!\r\n"
}  

/// extract path info from "/users/{userid}/{friend}" url
/// {number} -  - deserializes to a u32
pub fn echo_handler() -> Result<String> {
    let my_uuid = Uuid::new_v4();
    debug!("Generated UUID{}", my_uuid);
    Ok(format!("{}", my_uuid))
}

/// Returns a list of customer accounts as Json.
/// Receives no parameter.
pub fn customer_accounts_handler(db: web::Data<Pool<SqliteConnectionManager>>,) 
-> impl Future<Item = HttpResponse, Error = Error> {
    
    web::block(move || {
        let conn = db.get().unwrap();
        let mut stmt = conn.prepare("select * from customer_account").unwrap();

        let ca_iter = stmt.query_map(NO_PARAMS, |row| {
            Ok(CustomerAccount {
                id: row.get(0)?,
                name: row.get(1)?,
                user_name: row.get(2)?,
            })
        }).unwrap();

        let mut v: Vec<CustomerAccount> = Vec::new();

        for customer_account in ca_iter {
            let ca: CustomerAccount = customer_account.unwrap();
            debug!("Found accounts {:?}", ca);
            v.push(ca);
        };

        serde_json::to_string(&CustomerAccounts{customer_acount_list: v,})
    })
    .then(|res| match res {
        Ok(accounts) => {
            Ok(HttpResponse::Ok().json(accounts))
        },
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })

}

