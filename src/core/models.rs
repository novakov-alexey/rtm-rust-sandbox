use chrono::NaiveDateTime;
use core::schema::tasks;

#[derive(Identifiable, Queryable, Debug, PartialEq, Insertable, Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub added: NaiveDateTime,
    pub due: NaiveDateTime,
    pub list: String,
    pub notes: String,
    pub completed: bool,
    pub priority: String,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask {
    pub title: String,
    pub added: NaiveDateTime,
    pub due: NaiveDateTime,
    pub list: String,
    pub notes: String,
    pub priority: String,
}
