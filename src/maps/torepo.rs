// torepo.rs - Function modules being used to add EDI mapping specification records
// to the repository (2021-07-01 bar8tl)
use crate::maps::proc_maps::CrTp;
use rusqlite::Connection;

pub fn init_cr_repo (cnn: &Connection, cr: &CrTp) {
  cnn.execute("DELETE FROM mapspecs where mapid=?1 and chgnr=?2;",
    (&cr.mapid, &cr.chgnr)).expect("Table not reset");
  cnn.execute("DELETE FROM headers  where mapid=?1 and chgnr=?2;",
    (&cr.mapid, &cr.chgnr)).expect("Table not reset");
  cnn.execute("DELETE FROM groups   where mapid=?1 and chgnr=?2;",
    (&cr.mapid, &cr.chgnr)).expect("Table not reset");
  cnn.execute("DELETE FROM segments where mapid=?1 and chgnr=?2;",
    (&cr.mapid, &cr.chgnr)).expect("Table not reset");
  cnn.execute("DELETE FROM fields   where mapid=?1 and chgnr=?2;",
    (&cr.mapid, &cr.chgnr)).expect("Table not reset");
}

pub fn isrt_crhdr_repo(cnn: &Connection, cr: &CrTp, lstup: &String, seqno: &String) {
  cnn.execute("INSERT INTO headers VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
    (&cr.mapid, &cr.chgnr, &cr.hdr.mptit, lstup, &cr.hdr.authr, &cr.hdr.bvers,
     &cr.hdr.custm, &cr.hdr.tform, &cr.hdr.sform, &cr.rowno, seqno))
    .expect("Header row not inserted");
  cnn.execute("INSERT INTO mapspecs VALUES (?1,?2,?3,?4,?5,?6,?7)",
    (&cr.mapid, &cr.chgnr, &"".to_string(), &"".to_string(), &"".to_string(),
     &cr.rowno, &seqno)).expect("Mapspecs row not inserted");
}

pub fn isrt_cregrp_repo(cnn: &Connection, cr: &CrTp, seqno: &String) {
  cnn.execute("INSERT INTO groups VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
    (&cr.mapid, &cr.chgnr, &cr.ingrp, &"".to_string(), &"".to_string(),
     &"".to_string(), &"".to_string(), &"".to_string(), &cr.rowno, &seqno))
    .expect("Section row not inserted");
  cnn.execute("INSERT INTO mapspecs VALUES (?1,?2,?3,?4,?5,?6,?7)",
    (&cr.mapid, &cr.chgnr, &cr.ingrp, &"".to_string(), &"".to_string(),
     &cr.rowno, &seqno)).expect("Mapspecs row not inserted");
}

pub fn isrt_crgrps_repo(cnn: &Connection, cl: &[String; 7], cr: &CrTp,
  seqno: &String) {
  cnn.execute("INSERT INTO groups VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
    (&cr.mapid, &cr.chgnr, &cr.ingrp, &cl[3], &cl[4], &cl[5], &cl[0], &cl[1],
     &cr.rowno, &seqno))
    .expect("Section row not inserted");
  cnn.execute("INSERT INTO mapspecs VALUES (?1,?2,?3,?4,?5,?6,?7)",
    (&cr.mapid, &cr.chgnr, &cr.ingrp, &"".to_string(), &"".to_string(),
     &cr.rowno, &seqno)).expect("Mapspecs row not inserted");
}

pub fn isrt_crsgms_repo(cnn: &Connection, cl: &[String; 7], cr: &CrTp, sgmtp: &String,
  seqno: &String) {
  cnn.execute("INSERT INTO segments VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
    (&cr.mapid, &cr.chgnr, &cr.ingrp, &cr.insgm, &sgmtp, &cl[4], &cl[5], &cl[0],
     &cl[1], &cr.rowno, &seqno))
    .expect("Segment row not inserted");
  cnn.execute("INSERT INTO mapspecs VALUES (?1,?2,?3,?4,?5,?6,?7)",
    (&cr.mapid, &cr.chgnr, &cr.ingrp, &cr.insgm, &"".to_string(),
     &cr.rowno, &seqno)).expect("Mapspecs row not inserted");
}

pub fn isrt_crflds_repo(cnn: &Connection, cl: &[String; 7], cr: &CrTp,
  seqno: &String) {
  cnn.execute(
    "INSERT INTO fields VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)",
    (&cr.mapid, &cr.chgnr, &cr.ingrp, &cr.insgm, &cl[2], &cl[3], &cl[4], &cl[5],
     &cl[0], &cl[1], &cr.rowno, &seqno, &cl[6]))
    .expect("Field row not inserted");
  cnn.execute("INSERT INTO mapspecs VALUES (?1,?2,?3,?4,?5,?6,?7)",
    (&cr.mapid, &cr.chgnr, &cr.ingrp, &cr.insgm, &cl[2],
     &cr.rowno, &seqno)).expect("Mapspecs row not inserted");
}
