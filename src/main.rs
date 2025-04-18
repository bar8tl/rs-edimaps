// main.rs - Main entry point to the edimaps program (edi mappings to/from idocs)
// (2021-07-01 bar8tl)
mod assets;
mod config;
mod definitn;
mod maps;
mod readidoc;
mod reposit;

use crate::assets::{add_cdcodes, add_cddata, add_idoctp, add_index, add_wkflow};
use crate::config::get_config;
use crate::definitn::add_definitn;
use crate::maps::proc_maps::proc_maps;
use crate::maps::out_maps::out_maps;
use crate::readidoc::read_idocs::read_idocs;
use crate::reposit::ini_repo;

include!("args.rs");

const CONFIG_FILENAME: &str = ".\\.edimapsrc";

fn main() {
  let cli = Cli::parse();
  let mut rc = get_config(CONFIG_FILENAME);
  match &cli.command {
    Some(Commands::Init{ file }) => {
      ini_repo(&rc.general.dbopt, file);
    }
    Some(Commands::Add{ file, refer, def }) => {
             if *refer && *file == "cdcodes".to_string() {
        add_cdcodes (&rc.general.dbopt, format!("{}_codes.json",  rc.refers.refdr));
      } else if *refer && *file == "cddata".to_string()  {
        add_cddata  (&rc.general.dbopt, format!("{}_transp.json", rc.refers.refdr));
      } else if *refer && *file == "index".to_string()   {
        add_index   (&rc.general.dbopt, &rc.refers.idxpt,        &rc.refers.tabid);
      } else if *refer && *file == "idoctp".to_string()  {
        add_idoctp  (&rc.general.dbopt, format!("{}idoctp.json",  rc.refers.refdr));
      } else if *refer && *file == "wkflow".to_string()  {
        add_wkflow  (&rc.general.dbopt, format!("{}wkflow.json",  rc.refers.refdr));
      } else if *def {
        add_definitn(&rc.general.dbopt, &format!("{}{}",          rc.refers.defdr, *file));
      }
    }
    Some(Commands::Map{ file, repo, json, text }) => {
      let mapid = file.to_string();
      let flds: Vec<&str> = mapid.split('.').collect();
      rc.maps.mapid = flds[0].to_string();
      if flds.len() > 1 {
        rc.maps.chgnr = flds[1].to_string();
      }
      proc_maps(&rc.general.dbopt, &rc.refers, &rc.maps, *repo, *json, *text);
    }
    Some(Commands::Out{ templ, list, count }) => {
      let mut omode: String = "count".to_string();
      if *list  { omode = "list" .to_string(); }
      if *count { omode = "count".to_string(); }
      out_maps(&rc.general.dbopt, &rc.refers, &rc.maps, templ.to_string(), omode);
    }
    Some(Commands::Step{ stage, file, single, batch:_ }) => {
      read_idocs(&rc.general.dbopt, stage, &rc.wkflow, &file.to_string(), *single);
    }
    None => {}
  }
}
