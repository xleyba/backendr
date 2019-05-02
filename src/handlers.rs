use actix_web::{Result, Error, web, HttpResponse};
use actix_web::web::Query;
use futures::Future;

use uuid::Uuid;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{NO_PARAMS};

mod models;
use crate::handlers::models::CustomerAccount;
use crate::handlers::models::CustomerAccounts;

#[derive(Deserialize)]
pub struct Parameters {
    accountId: String,
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

        let ca = conn.query_row("SELECT * FROM customer_account WHERE id=$1", &[&account_id], |row| {
            Ok(CustomerAccount {
                id: row.get(0)?,
                name: row.get(1)?,
                user_name: row.get(2)?,
            })
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

/*
// Return data of requested account.
// Receives accountId.
func customerAccountHandler(db *sql.DB) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {

		if len(r.FormValue("accountId")) == 0 {
			http.Error(w, "Parameter accountId not present", http.StatusBadRequest)
			log.Error().Msg("Parameter accountId not present")
			return
		}

		rows, err := db.Query("select * from customer_account where id = " + r.FormValue("accountId"))
		if err != nil {
			http.Error(w, "Error from DB", http.StatusBadRequest)
			log.Error().Msgf("Error from DB %s", err.Error())
			return
		}
		defer rows.Close()

		var ca CustomerAccount

		for rows.Next() {

			err = rows.Scan(&ca.Id, &ca.Name, &ca.Username)
			if err != nil {
				log.Error().Msgf("Error: %s", err)
			}

		}

		err = rows.Err()
		if err != nil {
			http.Error(w, "Error from queries", http.StatusInternalServerError)
			log.Error().Msgf("%s", err)
			return
		}

		if err := json.NewEncoder(w).Encode(ca); err != nil {
			log.Error().Msgf("%s", err.Error())
			http.Error(w, "Error encoding JSON", http.StatusInternalServerError)
		}

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)

		log.Debug().Msgf("Returning: %v\n", ca)
	}

}*/