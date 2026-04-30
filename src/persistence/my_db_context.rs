use rusqlite::{Connection, Error, Statement, named_params};
pub struct MyDbContext<'a> {
    pub conn: &'a Connection,
}

impl<'a> MyDbContext<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        MyDbContext {
            conn,
        }
    }
}