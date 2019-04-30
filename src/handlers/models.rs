
pub struct CustomerAccount {
        pub id: i32,
        pub name: String,
        pub user_name: String,
    }

    pub struct CustomerAccountDetails {
        pub id: i32,
        pub name: String,
        pub user_name: String,
        pub movements: float32,
        pub total_amount: float32,
    }

    pub struct CustomerAccountMovement {
        pub id: i32,
        pub movement_date: String,
        pub amount: float32,
        pub concept: String,
        pub customer_account_id: i32,
    }

    pub struct CustomerAccountBalance {
        pub customer_account_id: i32,
        pub balance: float32,
    }