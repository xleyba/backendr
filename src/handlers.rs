use actix_web::{Result, web, HttpResponse};
use actix_web::web::Json;

use uuid::Uuid;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{NO_PARAMS};

mod models;
use crate::handlers::models::CustomerAccount;
use crate::handlers::models::CustomerAccounts;

pub struct Parameters {
    pub con: Pool<SqliteConnectionManager>,
}

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

/// Handle customerAccounts path
// Return all the accounts.
// Receives no parameters.
pub fn customer_accounts_handler(parameters: web::Data<Parameters>) 
//-> Result<Json<CustomerAccounts>> {
-> Result<String> {

    let conn = parameters.con.get().unwrap();
    
    let stmt = conn.prepare("select * from customer_account");

    let mut s = match stmt {
        Ok(stmt) => stmt,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    let ca_iter = s.query_map(NO_PARAMS, |row| {
        Ok(CustomerAccount {
            id: row.get(0)?,
            name: row.get(1)?,
            user_name: row.get(2)?,
            }
        )
    });

    let cal_iter = match ca_iter {
        Ok(ca_iter) => ca_iter,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    let mut v: Vec<CustomerAccount> = Vec::new();

    for customer_account in cal_iter {
        let ca: CustomerAccount = customer_account.unwrap();
        debug!("Found accounts {:?}", ca);
        v.push(ca);
    }


    let myresult = serde_json::to_string(&CustomerAccounts{customer_acount_list: v,})?;

    //Ok(Json(CustomerAccounts{customer_acount_list: v,}))
    //Ok("kk".to_string())
    Ok(myresult)
}
