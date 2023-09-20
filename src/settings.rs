//**********************************************************************************
// settings.rs : Define pgm-level & run-level settings (2019-07-01 bar8tl)
//**********************************************************************************
#![allow(unused)]

use crate::settings::config::ConfigTp;
use crate::utils::{IdxkeyTp, read_index};
use chrono::Local;
use chrono::NaiveDateTime;
use rblib::params::{ParamsTp, ParameTp};
use rusqlite::Connection;

const DEFAULTS: &str = include!("defaults.json");

#[derive(Debug, Clone, Default)]
pub struct SettingsTp {
  pub prm  : ParamsTp,
  pub cfd  : ConfigTp,
  pub dfl  : ConfigTp,
  pub dbonm: String,
  pub dbodr: String,
  pub dbopt: String,
  pub inpdr: String,
  pub inppt: String,
  pub outdr: String,
  pub mapdr: String,
  pub idxpt: String,
  pub trims: String,
  pub nodat: String,
  pub omite: String,
  pub ndchr: String,
  pub lfchr: String,
  pub objnm: String,
  pub sbobj: String,
  pub tabid: String,
  pub activ: String,
  pub mapid: String,
  pub ctmrs: String,
  pub ctmrl: String,
  pub messg: String,
  pub mvers: String,
  pub idocm: String,
  pub mstat: String,
  pub fname: String,
  pub relsd: String,
  pub chgnr: String,
  pub suprt: String,
  pub asgnd: String,
  pub dstat: String,
  pub templ: String,
  pub msgtp: String,
  pub dtsys: NaiveDateTime,
  pub dtcur: NaiveDateTime,
  pub dtnul: NaiveDateTime
}

impl SettingsTp {
  pub fn new_settings() -> SettingsTp {
    let mut stg = SettingsTp { ..Default::default() };
    stg.dfl = serde_json::from_str(DEFAULTS).unwrap();
    stg.prm = ParamsTp::new_params();
    stg.cfd = ConfigTp::new_config();
    stg.set_settings("_config.json");
    stg
  }

  pub fn set_settings(&mut self, cfnam: &str) {
    self.prm.scan_params();
    self.cfd.get_config(cfnam);
    let c = &self.cfd;
    self.dbonm = if c.progm.dbonm.len() > 0
      { c.progm.dbonm.clone() } else { self.dfl.progm.dbonm.clone() };
    self.dbodr = if c.progm.dbodr.len() > 0
      { c.progm.dbodr.clone() } else { self.dfl.progm.dbodr.clone() };
    self.inpdr = if c.progm.inpdr.len() > 0
      { c.progm.inpdr.clone() } else { self.dfl.progm.inpdr.clone() };
    self.outdr = if c.progm.outdr.len() > 0
      { c.progm.outdr.clone() } else { self.dfl.progm.outdr.clone() };
    self.trims = if c.progm.trims.len() > 0
      { c.progm.trims.clone() } else { self.dfl.progm.trims.clone() };
    self.nodat = if c.progm.nodat.len() > 0
      { c.progm.nodat.clone() } else { self.dfl.progm.nodat.clone() };
    self.omite = if c.progm.omite.len() > 0
      { c.progm.omite.clone() } else { self.dfl.progm.omite.clone() };
    self.ndchr = if c.progm.ndchr.len() > 0
      { c.progm.ndchr.clone() } else { self.dfl.progm.ndchr.clone() };
    self.lfchr = if c.progm.lfchr.len() > 0
      { c.progm.lfchr.clone() } else { self.dfl.progm.lfchr.clone() };
    self.dbopt = format!("{}{}", self.dbodr, self.dbonm);
    self.dtsys = Local::now().naive_local();
    self.dtcur = Local::now().naive_local();
    self.dtnul = NaiveDateTime::MIN;
  }

  pub fn set_runvars(&mut self, p: &ParameTp) {
    if p.prm1.len() > 0 {
      self.objnm = p.prm1.clone();
    } else {
      panic!("Error: Not possible to determine Object name");
    }
    for run in &self.dfl.run {
      if p.optn == run.optcd && p.prm1 == run.objnm {
        if p.optn == "crt" || p.optn == "lrf" {
          if run.activ.len() > 0 { self.activ = run.activ.clone(); }
        }
        if p.optn == "lrf" || p.optn == "des" || p.optn == "dsp" {
          if run.inpdr.len() > 0 { self.inpdr = run.inpdr.clone(); }
          if run.fname.len() > 0 { self.fname = run.fname.clone(); }
          if run.tabid.len() > 0 { self.tabid = run.tabid.clone(); }
          if run.mapdr.len() > 0 { self.mapdr = run.mapdr.clone(); }
        }
        break;
      }
    }
    for run in &self.cfd.run {
      if p.optn == run.optcd && p.prm1 == run.objnm {
        if p.optn == "cdb" || p.optn == "lrf" {
          if run.activ.len() > 0 { self.activ = run.activ.clone(); }
        }
        if p.optn == "lrf" || p.optn == "des" || p.optn == "dsp" {
          if run.inpdr.len() > 0 { self.inpdr = run.inpdr.clone(); }
          if run.fname.len() > 0 { self.fname = run.fname.clone(); }
          if run.tabid.len() > 0 { self.tabid = run.tabid.clone(); }
          if run.mapdr.len() > 0 { self.mapdr = run.mapdr.clone(); }
        }
        break;
      }
    }
    if p.optn == "lrf" {
      self.inppt = format!("{}{}", self.inpdr, self.fname);
    }
    if p.optn == "des" || p.optn == "dsp" {
      self.idxpt = format!("{}{}", self.inpdr, self.fname);
      if p.prm2.len() > 0 {
        let flds: Vec<&str> = p.prm2.split('.').collect();
        self.objnm = flds[0].to_string();
        if flds.len() > 1 {
          self.sbobj = flds[1].to_string();
        }
      }
    }
    if p.optn == "des" {
      self.get_mapdetail();
    }
  }

  pub fn get_mapdetail(&mut self) {
    let indx = read_index(IdxkeyTp{
      mapid: self.objnm.clone(), chgnr: self.sbobj.clone(),
      idxpt: self.idxpt.clone(), tabid: self.tabid.clone()}, "SINGLE");
    (self.mapid, self.ctmrs, self.ctmrl, self.messg, self.mvers, self.idocm,
     self.idocm, self.mstat, self.fname, self.relsd, self.chgnr, self.suprt,
     self.asgnd, self.dstat, self.templ, self.msgtp) =
    (indx[0][0] .clone(), indx[0][1] .clone(), indx[0][2] .clone(),
     indx[0][3] .clone(), indx[0][4] .clone(), indx[0][5] .clone(),
     indx[0][6] .clone(), indx[0][7] .clone(), indx[0][8] .clone(),
     indx[0][9] .clone(), indx[0][10].clone(), indx[0][11].clone(),
     indx[0][12].clone(), indx[0][13].clone(), indx[0][14].clone(),
     indx[0][15].clone());
  }
}

//**********************************************************************************
// config.rs : Reads config file (2017-05-24 bar8tl)
//**********************************************************************************
mod config {
  use serde::Deserialize;
  use serde_json::from_reader;
  use std::fs::File;

  #[derive(Debug, Clone, Default, Deserialize)]
  pub struct ProgmTp {
    #[serde(default)]
    pub dbonm: String,
    #[serde(default)]
    pub dbodr: String,
    #[serde(default)]
    pub inpdr: String,
    #[serde(default)]
    pub outdr: String,
    #[serde(default)]
    pub idxdr: String,
    #[serde(default)]
    pub trims: String,
    #[serde(default)]
    pub nodat: String,
    #[serde(default)]
    pub omite: String,
    #[serde(default)]
    pub ndchr: String,
    #[serde(default)]
    pub lfchr: String,
    #[serde(default)]
    pub outpt: Vec<OutptTp>
  }

  #[derive(Debug, Clone, Default, Deserialize)]
  pub struct OutptTp {
    #[serde(default)]
    pub otype: String,
    #[serde(default)]
    pub activ: String,
    #[serde(default)]
    pub ofile: String
  }

  #[derive(Debug, Clone, Default, Deserialize)]
  pub struct RunTp {
    #[serde(default)]
    pub optcd: String,
    #[serde(default)]
    pub objnm: String,
    #[serde(default)]
    pub mapdr: String,
    #[serde(default)]
    pub inpdr: String,
    #[serde(default)]
    pub outdr: String,
    #[serde(default)]
    pub fname: String,
    #[serde(default)]
    pub tabid: String,
    #[serde(default)]
    pub activ: String
  }

  #[derive(Debug, Clone, Default, Deserialize)]
  pub struct ConfigTp {
    #[serde(default)]
    pub progm: ProgmTp,
    #[serde(default)]
    pub run:   Vec<RunTp>
  }

  impl ConfigTp {
    pub fn new_config() -> ConfigTp {
      let cfg = ConfigTp{ ..Default::default() };
      cfg
    }

    pub fn get_config(&mut self, fname: &str) {
      let f = File::open(fname).unwrap();
      let cfg: ConfigTp = from_reader(f).unwrap();
      self.progm = cfg.progm;
      self.run   = cfg.run;
    }
  }
}
