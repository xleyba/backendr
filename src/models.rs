

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

/*
// Defines a data type for CustomerAccount.
type CustomerAccount struct {
	Id       int    `json:"id"`
	Name     string `json:"name"`
	Username string `json:"username"`
}

type CustomerAccounts []CustomerAccount

type CustomerAccountDetails struct {
	Id          int     `json:"id"`
	Name        string  `json:"name"`
	Username    string  `json:"username"`
	Movements   float32 `json:"movements"`
	TotalAmount float32 `json:"totalamount"`
}

type CustomerAccountsDetails []CustomerAccountDetails

type CustomerAccountMovement struct {
	Id                int     `json:"id"`
	MovementDate      string  `json:"movementdate"`
	Amount            float32 `json:"amount"`
	Concept           string  `json:"concept"`
	CustomerAccountId int     `json:"customeraccountid"`
}

type CustomerAccountMovements []CustomerAccountMovement

type CustomerAccountBalance struct {
	CustomerAccountId int     `json:"customeraccountid"`
	Balance           float32 `json:"balance"`
}
*/