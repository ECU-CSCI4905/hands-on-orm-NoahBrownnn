#![allow(unused)]
#![allow(clippy::all)]

use diesel::prelude::*;
use crate::schema::*;

// Full User struct for reading/querying from the DB
#[derive(Queryable, Selectable, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub active: bool,
}

// NewUser struct for inserting into the DB
#[derive(Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub active: bool,
}
