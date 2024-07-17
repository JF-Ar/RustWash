use actix_web::{post, web, Responder, HttpResponse};
use crate::AppState;
use super::customers::customers_model::{CustomersModel, Customer};
use bcrypt::{hash, DEFAULT_COST};


#[post("wash/customers_create")]
pub async fn create_customer(app_state: web::Data<AppState>, customer: web::Json<Customer>)
    -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO customer (name, email, phone)
        VALUES ($1, $2, $3)
        RETURNING id, name, email, phone
        "#,
        customer.name,
        customer.email,
        customer.phone
    ).fetch_one(&app_state.postgres_client);

    match result.await {
        Ok(customer) => {
            // Hash the customer ID
            let hashed_id = hash(&customer.id.to_string(), DEFAULT_COST)
                .expect("Failed to hash customer id");

            HttpResponse::Ok().json(CustomersModel{
                id: hashed_id,
                name: customer.name,
                email: customer.email.expect("Email is required"),
                phone: customer.phone.expect("Phone is required")
            })
        },
        Err(_) => HttpResponse::InternalServerError().body("Failed to create customer")
    }
}