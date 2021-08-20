use actix_web::web;

use crate::controller::SomethingController;

pub struct SomethingRoutes {
    controller: SomethingController,
}

impl SomethingRoutes {
    pub fn new(controller: SomethingController) -> SomethingRoutes {
        SomethingRoutes { controller }
    }

    pub fn something_routes(&self, server: &mut web::ServiceConfig) {
        server
            .service(web::resource("/api/something").route(web::post().to(crate::controller::get)));
    }
}
