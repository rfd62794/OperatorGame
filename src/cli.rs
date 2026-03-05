/// cli.rs — Clap derive-macro command tree for the OPERATOR CLI.
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "operator",
    about = "OPERATOR: A dispatch simulator. Assemble squads. Run missions. Collect rewards.",
    version = "0.1.0"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all operators on your roster and their current state.
    Roster,

    /// Hire a new operator and add them to the roster.
    Hire {
        /// Display name for the operator.
        name: String,
        /// Job specialisation: breacher | infiltrator | analyst
        #[arg(value_parser = parse_job)]
        job: crate::models::Job,
    },

    /// Show all available missions.
    Missions,

    /// Deploy a squad on a mission.
    Deploy {
        /// Short mission ID prefix (first 8 chars of UUID).
        mission_id_prefix: String,
        /// Operator ID prefix(es) — 1 to 3 operators.
        #[arg(required = true, num_args = 1..=3)]
        operator_id_prefixes: Vec<String>,
    },

    /// Resolve all completed deployments and collect rewards.
    Aar,

    /// Show bank balance and active deployments.
    Status,

    /// Launch the graphical War Room dashboard.
    Gui,
}

fn parse_job(s: &str) -> Result<crate::models::Job, String> {
    match s.to_lowercase().as_str() {
        "breacher" | "b" => Ok(crate::models::Job::Breacher),
        "infiltrator" | "i" => Ok(crate::models::Job::Infiltrator),
        "analyst" | "a" => Ok(crate::models::Job::Analyst),
        _ => Err(format!(
            "Unknown job '{}'. Choose: breacher, infiltrator, analyst",
            s
        )),
    }
}
