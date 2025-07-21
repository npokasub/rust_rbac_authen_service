use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone)]
pub struct DatabasePool;

impl DatabasePool {
  pub fn new(database_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(16)
        .build(manager)
        .expect("Could not build connection pool")
  }
}