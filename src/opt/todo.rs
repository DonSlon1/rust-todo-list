use std::str::FromStr;
use clap::Error;
use crate::db;
use clap::error::ErrorKind;
use rusqlite::{ToSql, params};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};

#[derive(Debug)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub status: Status
}

#[derive(Debug)]
pub enum Status {
    Completed,
    InProgress,
    NotStarted
}

impl Todo {
    pub fn load(show_completed: Option<bool>) -> Result<Vec<Todo>, ()>
    {
        let mut cond = "";
        match show_completed {
            Some(false) => cond = " WHERE status != 'COMPLETED'",
            _ => ()
        }
        let conn = db::connect();
        let sql = "SELECT id , name, status, description as data FROM todo".to_owned() +cond;
        let mut stm = conn.prepare(&*sql).unwrap();
        let rows = stm.query_map([], |row| {
            Ok(Todo {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                description: row.get(3).unwrap(),
                status: row.get(2).unwrap()
            })
        }).unwrap();
        let mut res:Vec<Todo> = Vec::new();
        for row in rows {
            res.push(row.unwrap());
        };
        Ok(res)
    }
    pub fn show(show_completed: Option<bool>) {
        let todos = Self::load(show_completed).unwrap();
        for todo in todos {
            println!("{}: {} - {} - {}", todo.id, todo.name, todo.description, todo.status.as_str());
        }
    }

    pub fn add(name: String, description: String, status: Status)  {
        let conn = db::connect();
        let mut stmt = conn.prepare("INSERT INTO todo (name, description, status) VALUES (?1, ?2, ?3)").unwrap();
        stmt.execute(params![name, description, status]).unwrap();
    }

    pub fn change_status(id: i32, status: Status) {
        let conn = db::connect();
        let mut stmt = conn.prepare("UPDATE todo set status = ?1 WHERE id = ?2").unwrap();
        stmt.execute(params![status, id]).unwrap();
    }


}

impl Clone for Todo {
    fn clone(&self) -> Self {
        Todo {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            status: self.status.clone()
        }
    }
}

impl Clone for Status {
    fn clone(&self) -> Self {
        match self {
            Status::Completed => Status::Completed,
            Status::InProgress => Status::InProgress,
            Status::NotStarted => Status::NotStarted
        }
    }
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Status::Completed => "COMPLETED",
            Status::InProgress => "IN_PROGRESS",
            Status::NotStarted => "NOT_STARTED"
        }
    }
    fn from_str_generic<T: AsRef<str>>(input: T) -> Result<Status, Error> {
        match input.as_ref() {
            "COMPLETED" => Ok(Status::Completed),
            "IN_PROGRESS" => Ok(Status::InProgress),
            "NOT_STARTED" => Ok(Status::NotStarted),
            _ => Err(Error::new(ErrorKind::InvalidValue)),
        }
    }

}

impl ToSql for Status {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_str()))
    }

}

impl FromStr for Status {
    type Err = Error;

    fn from_str(input: &str) -> Result<Status, Self::Err> {
        Status::from_str_generic(input)
    }
}

impl FromSql for  Status {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Status::from_str_generic(value.as_str()?).map_err(|_| FromSqlError::InvalidType)
    }
}
