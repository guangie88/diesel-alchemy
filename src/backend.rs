use diesel::{
    backend::Backend,
    connection::Connection,
    mysql::{Mysql, MysqlConnection},
    pg::{Pg, PgConnection},
    sqlite::{Sqlite, SqliteConnection},
    ConnectionResult,
};

use query_builder::{MultiBindCollector, MultiQueryBuilder, MultiRawValue, MultiByteOrder};

#[derive(Debug)]
pub enum MultiBackend {
    Mysql(Mysql),
    Pg(Pg),
    Sqlite(Sqlite),
}

impl Backend for MultiBackend {
    type QueryBuilder = MultiQueryBuilder;
    type BindCollector = MultiBindCollector;
    type RawValue = MultiRawValue;
    type ByteOrder = MultiByteOrder;
}
