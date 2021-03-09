use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

// #[database("POSTGRES_ROCKET")]
// pub struct db_conn(diesel::pg::PgConnection);

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// pub fn establish_connection() -> PgConnection {
//     pub type PgPool = Pool<ConnectionManager<PgConnection>>;
//     pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

//     fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
//         let manager = ConnectionManager::<PgConnection>::new(database_url);
//         Pool::builder().build(manager)
//     }

//     pub fn establish_connection() -> PgPool {
//         dotenv().ok();
        
//         let database_url = env::var("DATABASE_URL")
//             .expect("DATABASE_URL must be set");

//         PgConnection::establish(&database_url)
//             .expect(&format!("Error connecting to {}", database_url));

//         init_pool(&database_url).expect("Failed to create pool")
//     }
// }