use actix_web::{Result, Error, web, HttpResponse};
use actix_web::web::Query;
use futures::Future;

use uuid::Uuid;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{NO_PARAMS};
use serde_rusqlite::from_row;
use serde_rusqlite::from_rows;

mod models;
use crate::handlers::models::CustomerAccount;
use crate::handlers::models::CustomerAccounts;
use crate::handlers::models::CustomerAccountDetails;

#[derive(Deserialize)]
pub struct Parameters {
    accountId: String,
}

// Handle index route
pub fn index() -> &'static str {
    "Hello world!\r\n"
}  

/// Return a v4 UUID
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
        // username retrieved as user_name to match Rust snake case name.
        let mut statement = conn.prepare("select id, name, username as user_name from customer_account").unwrap();
        
        let mut rows_iter = from_rows::<CustomerAccount>(statement.query(NO_PARAMS).unwrap());

        let mut v: Vec<CustomerAccount> = Vec::new();

        loop {
            match rows_iter.next() {
                None => break,
                Some(ca) => {
                    debug!("CA: {:?}", ca);
                    v.push(ca);
                },
            };
        }

        serde_json::to_string(&CustomerAccounts{customer_acount_list: v,})
    })
    .then(|res| match res {
        Ok(accounts) => {
            Ok(HttpResponse::Ok().json(accounts))
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json("500 - Internal Server Error")),
    })
}

/// Return data of requested account.
/// Receives accountId.
pub fn customer_account_handler(msg: Query<Parameters>, 
    db: web::Data<Pool<SqliteConnectionManager>>,) 
-> impl Future<Item = HttpResponse, Error = Error> {

    let account_id = msg.accountId.clone();

     web::block(move || {
        let conn = db.get().unwrap();
        //let mut stmt = conn.prepare("select * from customer_account where id = ?1",
            //params![account_id]).unwrap();

        let ca = conn.query_row("select id, name, username as user_name FROM customer_account WHERE id=$1", 
            &[&account_id], |row| {
                Ok(from_row::<CustomerAccount>(&row).unwrap())
        }).unwrap();
        
        serde_json::to_string(&ca)
     })
     .then(|res| match res {
        Ok(account) => {
            Ok(HttpResponse::Ok().json(account))
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json("500 - Internal Server Error")),
    })
}

/// Return data of requested account.
/// Receives accountId.
pub fn customer_account_detail_handler(msg: Query<Parameters>, 
    db: web::Data<Pool<SqliteConnectionManager>>,) 
-> impl Future<Item = HttpResponse, Error = Error> {

    // Get parameter accountId
    let account_id = msg.accountId.clone(); 

    // Prepare query statement
    let mut query = String::from("SELECT a.ID as id, a.NAME as name, a.USERNAME as user_name, ");
	query.push_str("count(m.id) as movements, SUM(m.AMOUNT) as total_amount ");
	query.push_str("FROM CUSTOMER_ACCOUNT a, CUSTOMER_ACCOUNT_MOVEMENTS m ");
	query.push_str("WHERE a.ID = ");
    query.push_str(&account_id);
	query.push_str(" AND a.ID = m.CUSTOMER_ACCOUNT_ID ");
	query.push_str("GROUP BY a.ID, a.NAME, a.USERNAME");

    web::block(move || {
        let conn = db.get().unwrap();                               // get connection
        let mut stmt = conn.prepare(&query).unwrap();               // Set statement

        let ca = stmt.query_row(NO_PARAMS, |row| {
            Ok(from_row::<CustomerAccountDetails>(&row).unwrap())   // Serialize result
        }).unwrap();

        serde_json::to_string(&ca)                                  // return json as string
    })
    .then(|res| match res {
        Ok(account_details) => {
            Ok(HttpResponse::Ok().json(account_details))
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json("500 - Internal Server Error")),
    })       

}
