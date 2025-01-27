use super::{Group, User};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, MySqlPool};
use std::sync::Arc;

pub struct Table<'c, T>
where
    T: FromRow<'c, MySqlRow>,
{
    pub pool: Arc<MySqlPool>,
    _from_row: fn(&'c MySqlRow) -> sqlx::Result<T>,
}

impl<'c, T> Table<'c, T>
where
    T: FromRow<'c, MySqlRow>,
{
    fn new(pool: Arc<MySqlPool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
        }
    }
}

pub struct JoinTable<'c, T1, T2>
where
    T1: FromRow<'c, MySqlRow>,
    T2: FromRow<'c, MySqlRow>,
{
    pub pool: Arc<MySqlPool>,
    _from_row: (
        fn(&'c MySqlRow) -> sqlx::Result<T1>,
        fn(&'c MySqlRow) -> sqlx::Result<T2>,
    ),
}

impl<'c, T1, T2> JoinTable<'c, T1, T2>
where
    T1: FromRow<'c, MySqlRow>,
    T2: FromRow<'c, MySqlRow>,
{
    fn new(pool: Arc<MySqlPool>) -> Self {
        JoinTable {
            pool,
            _from_row: (T1::from_row, T2::from_row),
        }
    }
}

pub struct Database<'c> {
    pub users: Arc<Table<'c, User>>,
    pub groups: Arc<Table<'c, Group>>,
    pub users_to_groups: Arc<JoinTable<'c, User, Group>>,
}

impl Database<'_> {
    pub async fn new(sql_url: &str) -> Database<'_> {
        let pool = MySqlPool::connect(sql_url).await.unwrap();
        let pool = Arc::new(pool);

        Database {
            users: Arc::from(Table::new(pool.clone())),
            groups: Arc::from(Table::new(pool.clone())),
            users_to_groups: Arc::from(JoinTable::new(pool)),
        }
    }
}
