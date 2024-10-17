use crate::ipc::http::entity::ApiResponse;
use crate::ipc::service::ApiService;
use actix_web::web::Data;
use actix_web::{App, HttpResponse, HttpServer};
use std::{net, thread};

#[actix_web::get("/api/current-info")]
async fn current_info(service: Data<ApiService>) -> HttpResponse {
    match service.current_info() {
        Ok(rs) => HttpResponse::Ok().json(ApiResponse::success(rs)),
        Err(e) => HttpResponse::Ok().json(ApiResponse::failed(format!("{e}"))),
    }
}

pub async fn start(port: u16, api_service: ApiService) -> anyhow::Result<()> {
    let listener = net::TcpListener::bind(format!("127.0.0.1:{port}"))?;
    thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                if let Err(e) = start0(listener, api_service).await {
                    log::warn!("web api {e:?}")
                }
            });
    });
    Ok(())
}

async fn start0(listener: net::TcpListener, api_service: ApiService) -> anyhow::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(api_service.clone()))
            .service(current_info)
    })
    .listen(listener)?
    .run()
    .await?;
    Ok(())
}
