use rocket::State;
use rocket_contrib::Json;
use rtm::core::models::Task;
use rtm::core::service::TaskService;
use chrono::{NaiveDateTime, Duration, Utc};

type JsonOrError = Result<Json<Vec<Task>>, String>;

#[get("/")]
fn index() -> &'static str {
    "Hello, RTM!"
}

#[get("/tasks/today/<list>/<completed>")]
fn list_today(service: State<TaskService>, list: String, completed: bool) -> JsonOrError {
    let today = Utc::now().naive_local();
    tasks(&*service, &list, completed, Some(today))
}

#[get("/tasks/yesterday/<list>/<completed>")]
fn list_yesterday(service: State<TaskService>, list: String, completed: bool) -> JsonOrError {
    let yesterday = (Utc::now() - Duration::days(1)).naive_local();
    tasks(&*service, &list, completed, Some(yesterday))
}

#[get("/tasks/incomplete/<list>")]
fn list_incomplete(service: State<TaskService>, list: String) -> JsonOrError {
    tasks(&*service, &list, false, None)
}

fn tasks(service: &TaskService, list: &str, completed: bool, due: Option<NaiveDateTime>) -> JsonOrError {
    service.get_tasks(list, completed, due).map(|l| Json(l))
}