//**********************************************************************************
// main.rs: Starts processes for EDI-Mapping-Specs archiving (2019-07-01 bar8tl)
//**********************************************************************************
mod dbase;
mod mapspecs;
mod settings;
mod utils;

fn main() {
  let optns = ["cdb", "lrf", "des", "dsp"];
  let funcs = [dbase::crea_tables, dbase::load_refdata, mapspecs::deser_mapspec,
    dbase::dspl_mapspecs];
  let stg = settings::SettingsTp::new_settings();
  let t = stg.clone();
  for p in t.prm.cmdpr {
    let mut s = stg.clone();
    s.set_runvars(&p);
    funcs[optns.iter().position(|&x| x == p.optn).unwrap()](s);
  }
}
