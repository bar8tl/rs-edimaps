//**********************************************************************************
// main.rs: Starts processes for EDI mapping specs archiving (2019-07-01 bar8tl)
//**********************************************************************************
mod dbase;
mod mapspecs;
mod settings;

fn main() {
  let optns = ["cdb", "lrf", "des"];
  let funcs = [dbase::crea_tables, dbase::load_refdata, mapspecs::deser_mapspec];
  let stg = settings::SettingsTp::new_settings();
  let t = stg.clone();
  for p in t.prm.cmdpr {
    let mut s = stg.clone();
    s.set_runvars(&p);
    funcs[optns.iter().position(|&x| x == p.optn).unwrap()](s);
  }
}
