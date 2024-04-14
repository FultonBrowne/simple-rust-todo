use askama::Template;
use axum::{
    extract::Form,
    response::{Html, Redirect},
    routing::{get, post},
    Router,
};
use lazy_static::lazy_static;
use std::sync::Mutex;

// Define a global shared state for storing tasks
lazy_static! {
    static ref TASKS: Mutex<Vec<String>> = Mutex::new(vec![]);
}

// Define the HTML template using Askama
#[derive(Template)]
#[template(path = "todo.html")]

struct TodoListTemplate<'a> {
    tasks: &'a Vec<String>,
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(show_tasks))
        .route("/add", post(add_task));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// Handler to display tasks
async fn show_tasks() -> Html<String> {
    let tasks = TASKS.lock().unwrap();
    let template = TodoListTemplate { tasks: &tasks };
    Html(template.render().unwrap())
}

// Handler to add a new task
async fn add_task(Form(input): Form<AddTask>) -> Redirect {
    let mut tasks = TASKS.lock().unwrap();
    tasks.push(input.task);
    Redirect::to("/")
}

// Structure to receive form data
#[derive(serde::Deserialize)]
struct AddTask {
    task: String,
}
