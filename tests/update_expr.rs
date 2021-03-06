/*
 * Copyright (C) 2015  Boucher, Antoni <bouanto@zoho.com>
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#![feature(plugin, static_mutex)]
#![plugin(tql_macros)]

extern crate postgres;
extern crate tql;

use std::sync::{MUTEX_INIT, MutexGuard, StaticMutex};

use postgres::{Connection, SslMode};
use tql::{ForeignKey, PrimaryKey};

#[SqlTable]
#[allow(dead_code)]
struct TableUpdateExpr {
    id: PrimaryKey,
    field1: String,
    field2: i32,
    field3: i32,
    related_field: ForeignKey<RelatedTable>,
}

#[SqlTable]
#[allow(dead_code)]
struct RelatedTable {
    id: PrimaryKey,
    field1: String,
}

static LOCK: StaticMutex = MUTEX_INIT;

struct DatabaseMutex<'a> {
    connection: Connection,
    _data: MutexGuard<'a, ()>,
}

impl<'a> DatabaseMutex<'a> {
    fn new() -> DatabaseMutex<'a> {
        let data = LOCK.lock().unwrap();
        let connection = get_connection();
        let _ = sql!(RelatedTable.create());
        let _ = sql!(TableUpdateExpr.create());
        DatabaseMutex {
            connection: connection,
            _data: data,
        }
    }
}

impl<'a> Drop for DatabaseMutex<'a> {
    fn drop(&mut self) {
        let connection = &self.connection;
        let _ = sql!(TableUpdateExpr.drop());
        let _ = sql!(RelatedTable.drop());
    }
}

fn get_connection() -> Connection {
    Connection::connect("postgres://test:test@localhost/database", &SslMode::None).unwrap()
}

#[test]
fn test_update() {
    let table_mutex = DatabaseMutex::new();
    let connection = &table_mutex.connection;

    let id = sql!(RelatedTable.insert(field1 = "")).unwrap();
    let related_field = sql!(RelatedTable.get(id)).unwrap();

    let id = sql!(TableUpdateExpr.insert(field1 = "", field2 = 0, field3 = 0, related_field = related_field)).unwrap();

    let num_updated = sql!(TableUpdateExpr.get(id).update(field1 = "value1", field2 = 55)).unwrap();
    assert_eq!(1, num_updated);

    let table = sql!(TableUpdateExpr.get(id)).unwrap();
    assert_eq!(id, table.id);
    assert_eq!("value1", table.field1);
    assert_eq!(55, table.field2);

    let new_field2 = 42;
    let num_updated = sql!(TableUpdateExpr.filter(id == id).update(field1 = "test", field2 = new_field2)).unwrap();
    assert_eq!(1, num_updated);

    let table = sql!(TableUpdateExpr.get(id)).unwrap();
    assert_eq!(id, table.id);
    assert_eq!("test", table.field1);
    assert_eq!(42, table.field2);

    let new_id = sql!(TableUpdateExpr.insert(field1 = "", field2 = 0, field3 = 0, related_field = related_field)).unwrap();

    let num_updated = sql!(TableUpdateExpr.filter(id > new_id).update(field1 = "test", field2 = new_field2)).unwrap();
    assert_eq!(0, num_updated);

    let my_string = "my string";
    let new_field2 = 24;
    let num_updated = sql!(TableUpdateExpr.filter(id >= id && id <= new_id).update(field1 = my_string, field2 = new_field2)).unwrap();
    assert_eq!(2, num_updated);

    let table = sql!(TableUpdateExpr.get(id)).unwrap();
    assert_eq!(id, table.id);
    assert_eq!("my string", table.field1);
    assert_eq!(24, table.field2);

    let table = sql!(TableUpdateExpr.get(new_id)).unwrap();
    assert_eq!(new_id, table.id);
    assert_eq!("my string", table.field1);
    assert_eq!(24, table.field2);
}

#[test]
fn test_update_operation() {
    let table_mutex = DatabaseMutex::new();
    let connection = &table_mutex.connection;

    let id = sql!(RelatedTable.insert(field1 = "")).unwrap();
    let related_field = sql!(RelatedTable.get(id)).unwrap();

    let id = sql!(TableUpdateExpr.insert(field1 = "", field2 = 0, field3 = 1, related_field = related_field)).unwrap();

    let num_updated = sql!(TableUpdateExpr.get(id).update(field2 += 10)).unwrap();
    assert_eq!(1, num_updated);

    let table = sql!(TableUpdateExpr.get(id)).unwrap();
    assert_eq!(id, table.id);
    assert_eq!("", table.field1);
    assert_eq!(10, table.field2);
    assert_eq!(1, table.field3);

    let num_updated = sql!(TableUpdateExpr.get(id).update(field2 -= 3)).unwrap();
    assert_eq!(1, num_updated);

    let table = sql!(TableUpdateExpr.get(id)).unwrap();
    assert_eq!(id, table.id);
    assert_eq!("", table.field1);
    assert_eq!(7, table.field2);
    assert_eq!(1, table.field3);

    let num_updated = sql!(TableUpdateExpr.get(id).update(field2 *= 2)).unwrap();
    assert_eq!(1, num_updated);

    let table = sql!(TableUpdateExpr.get(id)).unwrap();
    assert_eq!(id, table.id);
    assert_eq!("", table.field1);
    assert_eq!(14, table.field2);
    assert_eq!(1, table.field3);

    let num_updated = sql!(TableUpdateExpr.get(id).update(field2 /= 3)).unwrap();
    assert_eq!(1, num_updated);

    let table = sql!(TableUpdateExpr.get(id)).unwrap();
    assert_eq!(id, table.id);
    assert_eq!("", table.field1);
    assert_eq!(4, table.field2);
    assert_eq!(1, table.field3);

    let num_updated = sql!(TableUpdateExpr.get(id).update(field2 += 10, field3 *= 3)).unwrap();
    assert_eq!(1, num_updated);

    let table = sql!(TableUpdateExpr.get(id)).unwrap();
    assert_eq!(id, table.id);
    assert_eq!("", table.field1);
    assert_eq!(14, table.field2);
    assert_eq!(3, table.field3);

    let num_updated = sql!(TableUpdateExpr.get(id).update(field2 %= 7)).unwrap();
    assert_eq!(1, num_updated);

    let table = sql!(TableUpdateExpr.get(id)).unwrap();
    assert_eq!(id, table.id);
    assert_eq!("", table.field1);
    assert_eq!(0, table.field2);
    assert_eq!(3, table.field3);
}
