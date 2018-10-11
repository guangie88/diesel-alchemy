use byteorder::ByteOrder;
use diesel::{
    mysql::Mysql,
    pg::{Pg, PgConnection},
    query_builder::{BindCollector, QueryBuilder},
    sqlite::Sqlite,
};

use backend::MultiBackend;

#[derive(Debug)]
pub enum MultiQueryBuilder {
    Mysql(Mysql),
    Pg(Pg),
    Sqlite(Sqlite),
}

#[derive(Debug)]
pub enum MultiBindCollector {
    Mysql(Mysql),
    Pg(Pg),
    Sqlite(Sqlite),
}

#[derive(Debug)]
pub enum MultiRawValue {
    Mysql(Mysql),
    Pg(Pg),
    Sqlite(Sqlite),
}

#[derive(Debug)]
pub enum MultiByteOrder {
    Mysql(Mysql),
    Pg(Pg),
    Sqlite(Sqlite),
}

impl QueryBuilder<MultiBackend> for MultiQueryBuilder {}

impl BindCollector<MultiBackend> for MultiBindCollector {}

impl ByteOrder for MultiByteOrder {
    fn read_u16(buf: &[u8]) -> u16 {
        unimplemented!();
    }
}
