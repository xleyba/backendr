#[derive(Deserialize,Serialize,Debug)]
pub struct CustomerAccount {
    pub id: i32,
    pub name: String,
    pub user_name: String,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct CustomerAccounts {
    pub customer_acount_list: Vec<CustomerAccount>,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct CustomerAccountDetails {
    pub id: i32,
    pub name: String,
    pub user_name: String,
    pub movements: f32,
    pub total_amount: f32,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct CustomerAccountMovement {
    pub id: i32,
    pub movement_date: String,
    pub amount: f32,
    pub concept: String,
    pub customer_account_id: i32,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct CustomerAccountMovements {
    pub customer_acount_mmnt_list: Vec<CustomerAccountMovement>,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct CustomerAccountBalance {
    pub customer_account_id: i32,
    pub balance: f32,
}
