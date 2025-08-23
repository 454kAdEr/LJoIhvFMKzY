use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, post, put, delete};

// Define a simple data structure to work with
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    id: u32,
    username: String,
    email: String,
}

// Define a fake database for demonstration
struct Db {
    users: Vec<User>,
}

// Implement the database structure
impl Db {
    fn new() -> Self {
        Self {
            users: Vec::new(),
        }
    }

    fn add_user(&mut self, user: User) -> &User {
        self.users.push(user);
        self.users.last().unwrap()
    }
}

// Define the handler for the GET /users endpoint
#[get("/users/{id}")]
async fn get_user(db: web::Data<Db>, id: web::Path<u32>) -> impl Responder {
    match db.users.iter().find(|u| u.id == *id) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}

// Define the handler for the POST /users endpoint
#[post("/users")]
async fn add_user_handler(db: web::Data<Db>, user: web::Json<User>) -> impl Responder {
    let user = db.add_user(user.into_inner());
    HttpResponse::Created().json(user)
}

// Define the handler for the PUT /users endpoint
#[put("/users/{id}")]
async fn update_user_handler(db: web::Data<Db>, id: web::Path<u32>, user: web::Json<User>) -> impl Responder {
    if let Some(index) = db.users.iter().position(|u| u.id == *id) {
        db.users[index] = user.into_inner();
        HttpResponse::Ok().json(&db.users[index])
    } else {
        HttpResponse::NotFound().finish()
    }
}

// Define the handler for the DELETE /users endpoint
#[delete("/users/{id}")]
async fn delete_user_handler(db: web::Data<Db>, id: web::Path<u32>) -> impl Responder {
    if let Some(index) = db.users.iter().position(|u| u.id == *id) {
        db.users.remove(index);
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Db::new()))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users", web::post().to(add_user_handler))
            .route("/users/{id}", web::put().to(update_user_handler))
            .route("/users/{id}", web::delete().to(delete_user_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
