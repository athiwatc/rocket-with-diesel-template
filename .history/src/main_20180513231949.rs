#![allow(unknown_lints)]
#![warn(clippy)]
#![feature(plugin)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv_codegen;

mod schema;
mod models;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::mysql::MysqlConnection;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

use self::models::*;
use self::diesel::prelude::*;


#[get("/")]
fn index(conn: DbConn) -> String {
    use schema::users::dsl::*;

    let all_users = users.load::<User>(&*conn).expect("BOOM");
    format!("{}", all_users.len())
}

#[get("/users/add/<username>")]
fn add_user(username: String, conn: DbConn) -> Json<User> {
    use schema::users::dsl::*;

    create_user(&*conn, &*username, &*token)
}

pub fn create_user<'a>(conn: &MysqlConnection, username: &'a str, token: &'a str) -> User {
    use schema::users;

    let new_user = NewUser {
        username,
        token,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}

fn main() {
    // dotenv::dotenv().ok();
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![index, add_user])
        .launch();
}

type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

fn init_pool() -> MysqlPool {
    let database_url = dotenv!("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::new(manager).expect("db pool")
}

pub struct DbConn(pub PooledConnection<ConnectionManager<MysqlConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<MysqlPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
