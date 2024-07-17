use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Customer {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>
}

#[derive(Serialize)]
pub struct CustomersModel {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: String
}