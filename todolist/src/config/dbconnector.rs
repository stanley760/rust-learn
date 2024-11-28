use bb8::Pool;
use bb8_sqlite::RusqliteConnectionManager;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<RusqliteConnectionManager>,
}


pub fn create_pool() -> AppState {
    let manager = RusqliteConnectionManager::new("./db.sqlite");
    let pool = Pool::builder().build(manager).unwrap();
    AppState {
        pool,
    }
}