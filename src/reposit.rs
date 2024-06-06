// reposit.rs - Function modules to create and maintain the data-files or
// db-tables of the edimaps program repository (2021-07-01 bar8tl)
use rblib::create_sqlite3_tablelist::{TlistTp, create_sqlite3_tablelist};
use rblib::create_sqlite3_table::create_sqlite3_table;
use serde::Deserialize;
use serde_json::from_str;

const ITABLES: &str = include!("sqlstmts.json");

#[derive(Debug, Clone, Default, Deserialize)]
struct SqlstTp {
  activ: String,
  table: String,
  sqlst: String
}

#[derive(Debug, Clone, Default, Deserialize)]
struct ItablesTp {
  sqlst: Vec<SqlstTp>
}

// ini_repo.rs - Sqlite3 DB tables creation for local IDOC definitions and EDI
// mapping specifications archive (2021-07-01 bar8tl)
// Command line: edimaps init [<table>|ALL]
pub fn ini_repo(dbopt: &String, table: &String) {
  let it: ItablesTp = from_str(ITABLES).unwrap();
  if table == "." || table == "*" || table.to_lowercase() == "all" {
    let mut tlist: Vec<TlistTp> = Vec::with_capacity(it.sqlst.len());
    for sql in &it.sqlst {
      if sql.activ.to_lowercase() == "yes"  {
        tlist.push(TlistTp {table: sql.table.clone(), sqlst: sql.sqlst.clone()});
      }
    }
    if tlist.len() > 0 {
      create_sqlite3_tablelist(dbopt, &tlist);
    }
  } else {
    for sql in &it.sqlst {
      if sql.table.as_str() == table && sql.activ.to_lowercase() == "yes" {
        create_sqlite3_table(dbopt, &sql.table, &sql.sqlst);
        break;
      }
    }
  }
}
