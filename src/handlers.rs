use actix_web::{Result, Error, web, HttpResponse};
use actix_web::web::Query;
use actix_http::{http, Response};
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
use crate::handlers::models::CustomerAccountMovement;
use crate::handlers::models::CustomerAccountMovements;

#[derive(Deserialize)]
pub struct Parameters {
    accountId: String,
}

#[derive(Deserialize)]
pub struct SortedParameters {
    accountId: String,
    sort: usize,
    asc: usize,
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
            Ok(Response::Ok()
                .set_header("X-TEST", "value")
                .set_header(http::header::CONTENT_TYPE, "application/json")
                .body(accounts)
            )
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
            Ok(HttpResponse::Ok().body(account))
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
            Ok(HttpResponse::Ok().body(account_details))
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json("500 - Internal Server Error")),
    })       

}

/// Retrieves all the customer account movements
/// Received parameters:
///		accountId - number with the account
/// 	sort	  - true or false or not present
///		asc		  - true or false or not present
/// Returns the list of movements sorted if requested
pub fn customer_account_movements_handler(msg: Query<SortedParameters>, 
    db: web::Data<Pool<SqliteConnectionManager>>,) 
-> impl Future<Item = HttpResponse, Error = Error> {

    // Get parameter accountId
    let account_id = msg.accountId.clone(); 

    // Prepare query statement
    let mut query = String::from("SELECT m.id, m.movement_date, m.amount, m.concept, m.customer_account_id ");
	query.push_str(" FROM CUSTOMER_ACCOUNT_MOVEMENTS m ");
	query.push_str(" WHERE m.CUSTOMER_ACCOUNT_ID = ");
    query.push_str(&account_id);

    web::block(move || {
        let conn = db.get().unwrap();                               // get connection
        let mut stmt = conn.prepare(&query).unwrap();               // Set statement

        let mut rows_iter = from_rows::<CustomerAccountMovement>(stmt.query(NO_PARAMS).unwrap());

        let mut v: Vec<CustomerAccountMovement> = Vec::new();

        loop {
            match rows_iter.next() {
                None => break,
                Some(cam) => {
                    debug!("CAM: {:?}", cam);
                    v.push(cam);
                },
            };
        }

        serde_json::to_string(&CustomerAccountMovements{customer_acount_mmnt_list: v,})                                  // return json as string
    })
    .then(|res| match res {
        Ok(account_details) => {
            Ok(HttpResponse::Ok().body(account_details))
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json("500 - Internal Server Error")),
    })  

}