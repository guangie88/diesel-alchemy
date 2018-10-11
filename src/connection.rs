use diesel::{
    connection::Connection, mysql::MysqlConnection, pg::PgConnection,
    sqlite::SqliteConnection, ConnectionResult,
};

pub enum MultiConnection {
    Mysql(MysqlConnection),
    Pg(PgConnection),
    Sqlite(SqliteConnection),
}

impl Connection for MultiConnection {
    fn establish(database_url: &str) -> ConnectionResult<Self> {}
}
