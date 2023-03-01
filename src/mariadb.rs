use diesel::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use diesel::r2d2::Pool;
use std::env;
 
// The Postgres-specific connection pool managing all database connections.
pub type MariaDBPool = Pool<ConnectionManager<MysqlConnection>>;
 
pub fn get_pool() -> MariaDBPool {
    // it from the environment within this function
    println!("Starting Pool Connection");
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("no DB URL");
    println!("{}",url);
    let migr = ConnectionManager::<MysqlConnection>::new(url);
    println!("Test");
    diesel::r2d2::Pool::builder()
        .build(migr)
        .expect("could not build connection pool")
}