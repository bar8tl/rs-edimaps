// args.rs - Structure of CLI subcommands and arguments being used in the EDIMAPS
// program (2021-07-01 bar8tl)
use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
  /// Initialize the repository
  Init {
    /// Table to add (ALL = all tables)
    file:  String,
  },
  /// Add data to the repository
  Add  {
    /// Reference file name
    file:  String,
    /// Reference data to be added to the repository
    #[arg(short, long)]
    refer: bool,
    /// Idoc definition to be added to the repository
    #[arg(short, long)]
    def:   bool,
  },
  /// Add mapping specifications to the repository and to the json files backup
  Map {
    /// Mapping specification file name
    file:  String,
    /// Add to repository
    #[arg(short, long)]
    repo:  bool,
    /// Generate JSON output file
    #[arg(short, long)]
    json: bool,
    /// Generate TXT output file
    #[arg(short, long)]
    text: bool,
  },
  /// Generates output of list and counters of records in mapping specifications
  Out {
    /// Mapping specifications template ID
    templ: String,
    /// List counters of each mapping specification file
    #[arg(short, long)]
    list:  bool,
    /// General counters of mapping specification
    #[arg(short, long)]
    count: bool,
  },
  /// Starts workflow of IDOC processes
  Step {
    /// Step code to be executed [fixed|json|query]
    stage: String,
    /// Idoc file name or Idocs folder name
    file : String,
    /// Run the step for a single file
    #[arg(short, long)]
    single: bool,
    /// Run the step for a folder (batch of files)
    #[arg(short, long)]
    batch:  bool,
  },
}
