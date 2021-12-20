use actix_files::Files;
use actix_web::{
    dev::Server, get, http::StatusCode, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use futures::{future::ok, stream::once};
use ssr_rs::Ssr;
use std::fs::{read_to_string, read_dir, read};
use std::net::TcpListener;
use std::sync::Mutex;

struct AppState {
    js_source: Mutex<String>,
    wasm_source: Mutex<Vec<u8>>
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        let mut wasm_path = String::new();
        
        for entry in read_dir("./web/dist/").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().unwrap().eq("wasm") {
                wasm_path = path.to_str().unwrap().to_string();
            }
        }

        let state = web::Data::new(AppState {
            js_source: Mutex::new(
                read_to_string("./web/dist/index.js").expect("Failed to load the JS resource."),
            ),
            wasm_source: Mutex::new(
                read(wasm_path).expect("Failed to load the WASM resource.")
            )
        });

        App::new()
            .app_data(state)
            .service(Files::new("/styles", "./web/dist/styles/").show_files_listing())
            .service(Files::new("/images", "./web/dist/images/").show_files_listing())
            .service(Files::new("/scripts", "./web/dist/scripts/").show_files_listing())
            .service(wasm)
            .service(index)
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[get("/{name}.wasm")]
async fn wasm(_req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let wasm = data.wasm_source.lock().unwrap();
    let body = once(ok::<_, Error>(web::Bytes::from(wasm.clone())));

    HttpResponse::build(StatusCode::OK)
        .content_type("application/wasm")
        .streaming(body)
}

#[get("*")]
async fn index(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let props = format!(
        r##"{{
            "location": "{}",
            "context": {{}}
        }}"##,
        req.uri()
    );

    let source = data.js_source.lock().unwrap();

    let response_body = Ssr::render_to_string(&source, "SSR", Some(&props));

    let body = once(ok::<_, Error>(web::Bytes::from(response_body)));

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .streaming(body)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
