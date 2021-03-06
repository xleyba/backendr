use actix_web::{Result, Error, web, HttpResponse};
use actix_web::web::Query;
use actix_http::http;
use futures::Future;

use uuid::Uuid;
use std::cmp::Reverse;

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
use crate::handlers::models::CustomerAccountBalance;


#[derive(Deserialize)]
pub struct Parameters {
    #[serde(rename = "accountId")]
    account_id: String,
}

#[derive(Deserialize)]
pub struct SortedParameters {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(default = "default_numeric")]
    sort: usize,
    #[serde(default = "default_numeric")]
    asc: usize,
}

#[derive(Deserialize)]
pub struct TopSortedParameters {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "totalElements", default = "default_elements")]
    total_elements: usize,
    #[serde(default = "default_numeric")]
    asc: usize,
}

// Will return default value for numeric parameters
fn default_numeric() -> usize {
    0
}

// Will return default value for total elements parameter
fn default_elements() -> usize {
    0
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
        
        let rows_iter = from_rows::<CustomerAccount>(statement.query(NO_PARAMS).unwrap());

        let v = rows_iter.collect::<Vec<CustomerAccount>>();
        
        /*let mut v: Vec<CustomerAccount> = Vec::new();

        loop {
            match rows_iter.next() {
                None => break,
                Some(ca) => {
                    debug!("CA: {:?}", ca);
                    v.push(ca);
                },
            };
        }*/

        serde_json::to_string(&CustomerAccounts{customer_acount_list: v,})
    })
    .then(|res| match res {
        Ok(accounts) => {
            Ok(HttpResponse::Ok()
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

    let account_id = msg.account_id.clone();

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
    let account_id = msg.account_id.clone(); 

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
    let account_id = msg.account_id.clone(); 

    // Prepare query statement
    let mut query = String::from("SELECT m.id, m.movement_date, m.amount, m.concept, m.customer_account_id ");
	query.push_str(" FROM CUSTOMER_ACCOUNT_MOVEMENTS m ");
	query.push_str(" WHERE m.CUSTOMER_ACCOUNT_ID = ");
    query.push_str(&account_id);

    web::block(move || {
        let conn = db.get().unwrap();                               // get connection
        let mut stmt = conn.prepare(&query).unwrap();               // Set statement

        let rows_iter = from_rows::<CustomerAccountMovement>(stmt.query(NO_PARAMS).unwrap());

        let mut v = rows_iter.collect::<Vec<CustomerAccountMovement>>();

        // Below commented code do the same than above collect
        // but performs slower.
        /*let mut v: Vec<CustomerAccountMovement> = Vec::new();

        loop {
            match rows_iter.next() {
                None => break,
                Some(cam) => {
                    debug!("CAM: {:?}", cam);
                    v.push(cam);
                },
            };
        }*/

        // Get parameter sort
        if msg.sort == 1 {
            if msg.asc == 0 {
                v.sort_by_key(|x| Reverse(x.id)); 
            } else {
                v.sort_by_key(|x| x.id); 
            }
        } 

        serde_json::to_string(&CustomerAccountMovements{customer_acount_mmnt_list: v,})                                  // return json as string
    })
    .then(|res| match res {
        Ok(account_mvmt) => {
            Ok(HttpResponse::Ok()
                .set_header("X-TEST", "value")
                .set_header(http::header::CONTENT_TYPE, "application/json")
                .body(account_mvmt)
            )
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json("500 - Internal Server Error")),
    })  
}

// Retrieves all the customer account movements but will display just the top
// requested after sort them if required.
//		accountId 		- number with the account
// 		totalElements	- number of rows to show
//		asc		  		- true or false or not present
// Returns the list of movements sorted if requested
pub fn customer_account_movements_top_handler(msg: Query<TopSortedParameters>, 
    db: web::Data<Pool<SqliteConnectionManager>>,) 
-> impl Future<Item = HttpResponse, Error = Error> {

    // Get parameter accountId
    let account_id = msg.account_id.clone(); 

    // Prepare query statement
    let mut query = String::from("SELECT m.id, m.movement_date, m.amount, m.concept, m.customer_account_id ");
	query.push_str(" FROM CUSTOMER_ACCOUNT_MOVEMENTS m ");
	query.push_str(" WHERE m.CUSTOMER_ACCOUNT_ID = ");
    query.push_str(&account_id);

    web::block(move || {
        let conn = db.get().unwrap();                               // get connection
        let mut stmt = conn.prepare(&query).unwrap();               // Set statement

        let mut rows_iter = from_rows::<CustomerAccountMovement>(stmt.query(NO_PARAMS).unwrap());

        // let mut v = rows_iter.collect::<Vec<CustomerAccountMovement>>();

        let mut v: Vec<CustomerAccountMovement> = Vec::new();

        // 
        for _x in 0..msg.total_elements {
            match rows_iter.next() {
                None => break,
                Some(cam) => {
                    debug!("CAM: {:?}", cam);
                    v.push(cam);
                },
            };
        }

        // Get parameter sort
        if msg.asc == 0 {
            v.sort_by_key(|x| Reverse(x.id)); 
        } else {
            v.sort_by_key(|x| x.id); 
        }

        serde_json::to_string(&CustomerAccountMovements{customer_acount_mmnt_list: v,})                                  // return json as string
    })
    .then(|res| match res {
        Ok(account_mvmt) => {
            Ok(HttpResponse::Ok()
                .set_header("X-TEST", "value")
                .set_header(http::header::CONTENT_TYPE, "application/json")
                .body(account_mvmt)
            )
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json("500 - Internal Server Error")),
    })  
}

// Retrieves all the customer account movements but will display just 
// the balance of the amounts
//		accountId 		- number with the account
// Returns a balance object
pub fn customer_account_movements_balance_handler(msg: Query<Parameters>, 
    db: web::Data<Pool<SqliteConnectionManager>>,) 
-> impl Future<Item = HttpResponse, Error = Error> {

    // Get parameter accountId
    let account_id = msg.account_id.clone(); 

    // Prepare query statement
    let mut query = String::from("SELECT m.id, m.movement_date, m.amount, m.concept, m.customer_account_id ");
	query.push_str(" FROM CUSTOMER_ACCOUNT_MOVEMENTS m ");
	query.push_str(" WHERE m.CUSTOMER_ACCOUNT_ID = ");
    query.push_str(&account_id);

    web::block(move || {
        let conn = db.get().unwrap();                               // get connection
        let mut stmt = conn.prepare(&query).unwrap();               // Set statement

        let mut rows_iter = from_rows::<CustomerAccountMovement>(stmt.query(NO_PARAMS).unwrap());

        let mut balance = 0f32;
        let mut ca_id = 0i32;

        loop {
            match rows_iter.next() {
                None => break,
                Some(cam) => {
                    debug!("CAM: {:?}", cam);
                    ca_id = cam.customer_account_id;
                    balance = balance + cam.amount;
                },
            };
        }

        serde_json::to_string(&CustomerAccountBalance{
            customer_account_id: ca_id,
            balance: balance,})                                  // return json as string
    })
    .then(|res| match res {
        Ok(account_mvmt) => {
            Ok(HttpResponse::Ok()
                .set_header("X-TEST", "value")
                .set_header(http::header::CONTENT_TYPE, "application/json")
                .body(account_mvmt)
            )
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json("500 - Internal Server Error")),
    })  
}    