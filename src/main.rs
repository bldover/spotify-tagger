use crate::db::establish_connection;

mod db;

fn main() {
    establish_connection();
    println!("Hello, world!");
}
