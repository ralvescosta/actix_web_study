use actix_web::web;

pub fn register(server: &mut web::ServiceConfig) {
    server.service(crate::controllers::something::index);
}
