// use diesel::prelude::*;
// use diesel::PgConnection;
// use dotenv::dotenv;
// use std::env;

// #[database("POSTGRES_ROCKET")]
// pub struct db_conn(diesel::PgConnection);

#[database("postgres_db")]
pub struct DBConnect(diesel::PgConnection);

// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
    
//     PgConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }