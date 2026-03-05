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

    // ── Genetics ─────────────────────────────────────────────────────────────

    /// List the slime stable (all genomes in your collection).
    Slimes,

    /// Hatch a new slime from a cultural archetype.
    Hatch {
        /// Name for the new slime.
        name: String,
        /// Culture: ember | gale | marsh | crystal | tundra | tide | void
        #[arg(value_parser = parse_culture)]
        culture: crate::genetics::Culture,
    },

    /// Splice two slimes to produce an offspring.
    Splice {
        /// Short ID prefix of the first parent (≥4 chars).
        parent_a_prefix: String,
        /// Short ID prefix of the second parent (≥4 chars).
        parent_b_prefix: String,
        /// Name for the offspring.
        offspring_name: String,
    },

    /// Collect finished genomes from the Bio-Incubator.
    Incubate,
}

fn parse_job(s: &str) -> Result<crate::models::Job, String> {
    match s.to_lowercase().as_str() {
        "breacher" | "b"    => Ok(crate::models::Job::Breacher),
        "infiltrator" | "i" => Ok(crate::models::Job::Infiltrator),
        "analyst" | "a"     => Ok(crate::models::Job::Analyst),
        _ => Err(format!("Unknown job '{}'. Choose: breacher, infiltrator, analyst", s)),
    }
}

fn parse_culture(s: &str) -> Result<crate::genetics::Culture, String> {
    match s.to_lowercase().as_str() {
        "ember"   | "e" => Ok(crate::genetics::Culture::Ember),
        "gale"    | "g" => Ok(crate::genetics::Culture::Gale),
        "marsh"   | "m" => Ok(crate::genetics::Culture::Marsh),
        "crystal" | "c" => Ok(crate::genetics::Culture::Crystal),
        "tundra"  | "t" => Ok(crate::genetics::Culture::Tundra),
        "tide"    | "d" => Ok(crate::genetics::Culture::Tide),
        "void"    | "v" => Ok(crate::genetics::Culture::Void),
        _ => Err(format!("Unknown culture '{}'. Choose: ember, gale, marsh, crystal, tundra, tide, void", s)),
    }
}
