// use diesel::prelude::*;
// use diesel::PgConnection;
// use dotenv::dotenv;
// use std::env;

// #[database("POSTGRES_ROCKET")]
// pub struct db_conn(diesel::PgConnection);

// #[database("postgres_db")]
// pub struct DBConnect(diesel::PgConnection);
#[database("redist_db")]
pub struct DBConnect(redis::Connection);

// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
    
//     PgConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }
use rocket::http;
use rocket::request;
use rocket::Outcome;
use rocket::State;
use r2d2;
use r2d2_redis::RedisConnectionManager;

const REDIS_ADDRESS: &'static str = "redis://localhost:6379";

// Pool initiation.
// Call it starting an app and store a pool as a rocket managed state.
pub fn pool() -> Pool {
    let manager = RedisConnectionManager::new(REDIS_ADDRESS).expect("connection manager");
    let redis_config = Default::default();

    r2d2::Pool::new(redis_config, manager).expect("db pool")
}

// Rocket guard type: a wrapper around an r2d2 pool.
// In conjunction with
// `impl<'a, 'r> request::FromRequest<'a, 'r> for RedisConnection` (see below)
// it allows code like:
//   ```
//   #[post("/<item>")]
//   fn create(item: &RawStr, connection: RedisConnection) -> ...
//
pub struct RedisConnection(pub r2d2::PooledConnection<RedisConnectionManager>);

// An alias to the type for a pool of redis connections.
type Pool = r2d2::Pool<RedisConnectionManager>;

// Retrieving a single connection from the managed database pool.
impl<'a, 'r> request::FromRequest<'a, 'r> for RedisConnection {
    type Error = ();

    fn from_request(request: &'a request::Request<'r>) -> request::Outcome<RedisConnection, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(RedisConnection(conn)),
            Err(_) => Outcome::Failure((http::Status::ServiceUnavailable, ())),
        }
    }
}