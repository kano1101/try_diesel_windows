#[macro_use]
extern crate diesel;

mod db;
mod model;
mod schema;

use diesel::prelude::*;
use diesel::result::Error;
// use diesel::MysqlConnection;
use dotenv::dotenv;
use std::env;
use transaction::prelude::*;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let conn = establish_connection();
    let tx = with_ctx(|ctx| -> Result<(), Error> {
        let user = db::create_user("keen").run(ctx)?;
        println!("created user: {:?}", user);
        let res = db::update_user(user.id, "KeenS").run(ctx)?;
        match res {
            None => {
                println!("user not found");
                return Ok(());
            }
            Some(()) => (),
        };
        let updated_user = match db::find_user(user.id).run(ctx)? {
            None => {
                println!("user not found");
                return Ok(());
            }
            Some(u) => u,
        };

        println!("updated user: {:?}", updated_user);
        match db::delete_user(updated_user.id).run(ctx)? {
            None => {
                println!("user not found");
            }
            Some(()) => (),
        };
        Ok(())
    });
    transaction_diesel_mysql::run(&conn, tx).unwrap()
}
