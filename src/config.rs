// config.rs - Reads the edimaps program configuration file and populate the settings
// to be used by function modules (2021-07-01 bar8tl)
use rblib::ownpath::ownpath;
use serde::Deserialize;
use std::fs::read_to_string;
use toml::from_str;

pub const PMODE: &str = "auto";
pub const TRIMS: &str = "no";
pub const NODAT: &str = "yes";
pub const OMITE: &str = "no";
pub const NDCHR: &str = "¤";
pub const LFCHR: &str = "\\n";
pub const CNTRL: &str = "EDI_DC40";
pub const CLIEN: &str = "011";
pub const RCVPF: &str = "RE";

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigTp {
  pub general: GeneralTp,
  pub refers : RefersTp,
  pub maps   : MapsTp,
  pub wkflow : WkflowTp
}

#[derive(Clone, Debug, Deserialize)]
pub struct GeneralTp {
  pub home : String,
  pub dbopt: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct RefersTp {
  pub refdr: String,
  pub idxpt: String,
  pub tabid: String,
  pub defdr: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct MapsTp {
  #[serde(default)]
  pub mapid: String,
  #[serde(default)]
  pub chgnr: String,
  #[serde(default)]
  pub mapdr: String,
  #[serde(default)]
  pub bkpdr: String,
  #[serde(default)]
  pub pmode: String,
  #[serde(default)]
  pub trims: String,
  #[serde(default)]
  pub nodat: String,
  #[serde(default)]
  pub omite: String,
  #[serde(default)]
  pub ndchr: String,
  #[serde(default)]
  pub lfchr: String
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct WkflowTp {
  #[serde(default)]
  pub cntrl: String,
  #[serde(default)]
  pub clien: String,
  #[serde(default)]
  pub rcvpf: String
}

pub fn get_config(fname: &str) -> ConfigTp {
  let confg = read_to_string(fname).expect("Failed to read file");
  let mut rc: ConfigTp = from_str(&confg).expect("Failed to deserialize");
  if rc.maps  .pmode.len() == 0 { rc.maps  .pmode = PMODE.to_string(); }
  if rc.maps  .trims.len() == 0 { rc.maps  .trims = TRIMS.to_string(); }
  if rc.maps  .nodat.len() == 0 { rc.maps  .nodat = NODAT.to_string(); }
  if rc.maps  .omite.len() == 0 { rc.maps  .omite = OMITE.to_string(); }
  if rc.maps  .ndchr.len() == 0 { rc.maps  .ndchr = NDCHR.to_string(); }
  if rc.maps  .lfchr.len() == 0 { rc.maps  .lfchr = LFCHR.to_string(); }
  if rc.wkflow.cntrl.len() == 0 { rc.wkflow.cntrl = CNTRL.to_string(); }
  if rc.wkflow.clien.len() == 0 { rc.wkflow.clien = CLIEN.to_string(); }
  if rc.wkflow.rcvpf.len() == 0 { rc.wkflow.rcvpf = RCVPF.to_string(); }
  rc.refers.refdr = ownpath(&rc.general.home, &rc.refers.refdr);
  rc.refers.idxpt = ownpath(&rc.general.home, &rc.refers.idxpt);
  rc.refers.defdr = ownpath(&rc.general.home, &rc.refers.defdr);
  rc.maps  .mapdr = ownpath(&rc.general.home, &rc.maps  .mapdr);
  rc.maps  .bkpdr = ownpath(&rc.general.home, &rc.maps  .bkpdr);
  return rc;
}
