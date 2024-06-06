// out_maps.rs - Generates output of mapping specification statistics
// (2021-07-01 bar8tl)
use crate::assets::{IdxkeyTp, read_index};
use crate::config::{RefersTp, MapsTp};
use rusqlite::Connection;

// out_maps - Produces statistics totals summary and list of records in mapping
// specs in order to check every file is processed okay (2019-07-01 bar8tl)
pub fn out_maps(dbopt: &String, rfr: &RefersTp, map: &MapsTp, templ: String,
  outtp: String) {
  let cnn  = Connection::open(dbopt).unwrap();
  let indx = read_index(IdxkeyTp{
    mapid: map.mapid.clone(), chgnr: map.chgnr.clone(), idxpt: rfr.idxpt.clone(),
    tabid: rfr.tabid.clone()}, "ALL");
  for c in indx {
    if c[14] == templ {
      if outtp == "count" {
        let mut count: usize = 0;
        cnn.query_row("SELECT count(*) FROM mapspecs WHERE mapid=?1 and chgnr=?2;",
          [c[0].to_string(), c[10].to_string()], |row| {
            Ok(count = row.get(0).unwrap()) })
          .expect("Error: Segment type not found in definition DB");
        println!("{},{},{}", c[0], c[10], count);
      } else {
        let mut stmt = cnn.prepare(
          "SELECT mapid,chgnr,grpid,sgmid,targt,rowno,seqno from mapspecs
            WHERE mapid=?1 and chgnr=?2;").unwrap();
        let mut rows = stmt.query([c[0].to_string(), c[10].to_string(),]).unwrap();
        while let Some(row) = rows.next().expect("while row failed") {
          let mapid: String = row.get(0).unwrap();
          let chgnr: String = row.get(1).unwrap();
          let grpid: String = row.get(2).unwrap();
          let sgmid: String = row.get(3).unwrap();
          let targt: String = row.get(4).unwrap();
          let rowno: String = row.get(5).unwrap();
          let seqno: String = row.get(6).unwrap();
          println!("{},{},{},{},{},{},{}", mapid, chgnr, grpid, sgmid, targt,
            rowno, seqno);
        }
      }
    }
  }
}
