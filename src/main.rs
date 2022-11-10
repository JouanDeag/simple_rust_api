use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::thread::spawn(|| {
        loop {
            println!("Waiting for commands...");
            // Read input for commands
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            // If the input is "exit", exit the program
            if input.trim() == "exit" {
                println!("Type 'confirm' to exit the program");
                let mut confirm: String = String::new();
                std::io::stdin().read_line(&mut confirm).unwrap();
                if confirm.trim() == "confirm" {
                    std::process::exit(0);
                } else {
                    continue;
                }
            }
        }
    });

    println!("Started Server on http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(ping)
            .service(web::resource("/manual_hello").to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))
    .expect("Can not bind to port 8080")
    .run()
    .await
}
