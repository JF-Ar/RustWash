pub mod model;
pub mod controllers;


pub fn customers_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(controllers::customers_controller::create_customer);
    //cfg.service(customers_controller::get_customers);
    //cfg.service(customers_controller::get_customer);
    //cfg.service(customers_controller::update_customer);
    //cfg.service(customers_controller::delete_customer);
}