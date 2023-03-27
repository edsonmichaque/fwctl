use clap::{Parser, Subcommand};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Zone(cmd) => match cmd {
            ZoneCmd::List { .. } => ZoneHandler::list(),
            ZoneCmd::Get { .. } => println!("get"),
            ZoneCmd::New { .. } => println!("new"),
            ZoneCmd::Delete { .. } => println!("delete"),
            ZoneCmd::Reset { .. } => println!("reset"),
        },
        Commands::Policy(_) => println!("policy"),
        Commands::Service => println!("service"),
        Commands::Ipset(..) => println!("ipset"),
        Commands::Icmtype => println!("icmtype"),
        Commands::Reload { .. } => println!("reload"),
        Commands::State => println!("state"),
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    verbose: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(subcommand)]
    Zone(ZoneCmd),

    #[command(subcommand)]
    Policy(PolicyCmd),

    Service,

    #[command(subcommand)]
    Ipset(IpsetCmd),

    Icmtype,
    Reload {
        #[arg(long)]
        complete: bool,
    },
    State,
}

#[derive(Subcommand)]
enum ZoneCmd {
    List {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        active: bool,
    },
    Get {
        zone: String,
    },
    New {
        zone: String,
        #[arg(long)]
        r#type: String,
    },
    Delete {
        zone: String,
    },
    Reset {
        zone: String,
    },
}

#[derive(Subcommand)]
enum IpsetCmd {
    List {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        active: bool,
    },
    Get {
        ipset: String,
    },
    New {
        ipset: String,
        #[arg(long)]
        r#type: String,
        #[arg(long)]
        option: Option<Vec<String>>,
    },
    Delete {
        ipset: String,
    },
    Reset {
        ipset: String,
    },
    #[command(subcommand)]
    Entry(EntryCmd),
}

#[derive(Subcommand)]
enum PolicyCmd {
    List {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        active: bool,
    },
    Get {
        policy: String,
    },
    New {
        policy: String,
    },
    Delete {
        policy: String,
    },
    Reset {
        policy: String,
    },
}

#[derive(Subcommand)]
enum EntryCmd {
    List {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        active: bool,
    },
    Get {
        entry: String,
    },
    New {
        entry: String,
    },
    Delete {
        entry: String,
    },
    Reset {
        entry: String,
    },
}

struct ZoneHandler {}

impl ZoneHandler {
    fn list() {
        println!("zone list")
    }
}
