use actix_web::{web, Responder};

use crate::{controllers::something::SomethingController, models::something::SomethingModel};

pub async fn index(body: web::Json<SomethingModel>) -> String {
    String::from("Oii, eu sou o Goku!")
}

pub fn routes() -> impl FnOnce(&mut web::ServiceConfig) {
    |server: &mut web::ServiceConfig| {
        println!("oi");
        server.service(web::resource("/").route(web::post().to(index)));
    }
}
