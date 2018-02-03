#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel; // Diesel libs
#[macro_use] extern crate diesel_codegen; // Code generation libs

extern crate dotenv; // Access to vars in .env file

extern crate r2d2; // DB connection pool
extern crate r2d2_diesel; // Diesel plugin

extern crate rocket; // Rocket libs
extern crate rocket_contrib; // Rocket APIS

#[macro_use] extern crate serde_derive;


pub mod models;
pub mod schema;


use dotenv::dotenv;
use diesel::prelude::*;
use r2d2::{Pool, PooledConnection}; // Config?
use r2d2_diesel::ConnectionManager;
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use std::env;
use std::ops::Deref;


pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  //let config = Config::default();
  let manager = ConnectionManager::<PgConnection>::new(database_url);

  Pool::new(manager).expect("Failed to create pool.")
}


pub struct DbConn(PooledConnection<ConnectionManager<PgConnection>>);

// Our DB connection is beneath two smart pointers, PooledConnection and ConnectionManager
impl Deref for DbConn {
  type Target = PgConnection;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
  type Error = (); // TODO: This may need to be something else

  fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
    let pool = request.guard::<State<Pool<ConnectionManager<PgConnection>>>>()?;

    match pool.get() {
      Ok(conn) => Outcome::Success(DbConn(conn)),
      Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
    }
  }
}