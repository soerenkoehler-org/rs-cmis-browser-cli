use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(multicall = true)]
pub(super) struct ReplCli {
    #[command(subcommand)]
    pub(super) cmd: Option<Command>,
}

#[derive(Subcommand)]
pub(super) enum Command {
    /// Connect to a CMIS server
    #[command()]
    Login {
        /// List the available connection configurations
        #[arg(short, long)]
        list: bool,
        /// Connection name as defined in the config file
        #[arg(conflicts_with = "list", required_unless_present = "list")]
        connection: Option<String>,
        /// Optional: repo to select after successful login
        #[arg(requires = "connection")]
        repo: Option<String>,
        /// Force interactive input of client secret
        #[arg(short, long, requires = "connection")]
        interactive: bool,
    },

    /// Switch to a repository in the current connection
    #[command()]
    Switch {
        /// List the available repositories
        #[arg(short, long)]
        list: bool,
        /// Repository name to switch into
        #[arg(conflicts_with = "list", required_unless_present = "list")]
        repo: Option<String>,
    },

    /// Change the remote working directory
    #[command()]
    Cd {
        /// The new path to change into (can be relative or absolute)
        #[arg()]
        path: String,
    },

    ///  FIXME unused List the content of the given remote directory
    // #[command()]
    // Dir {
    //     /// The path to the directory
    //     #[arg(default_value = ".")]
    //     path: String,
    // },

    /// List the properties of the given object
    #[command(visible_alias = "prop")]
    Stat {
        /// The path to the object
        #[arg(default_value = ".")]
        path: String,
    },

    /// A select query to the archive
    #[command()]
    Select {
        /// The query without the leading select
        #[arg()]
        query: Vec<String>,
        // Attention: This argument is solely for help display. Due to quoting
        // conflicts between shell and CMIS-SQL the command handler directly
        // processes the raw remainder of the input line.
    },

    /// (NOT IMPLEMENTED) Query the number of objects (shortcut to 'select count(*)')
    #[command()]
    Count {
        #[arg(short, long = "where")]
        where_clause: String,
    },

    /// (NOT IMPLEMENTED) Operate on the local filesystem
    #[command()]
    Local {
        #[command(subcommand)]
        cmd: LocalCommands,
    },

    /// Clear the terminal
    #[command(visible_alias = "cls")]
    Clear,

    /// Terminate the program
    #[command(visible_alias = "quit")]
    Exit,
}

#[derive(Subcommand)]
pub(super) enum LocalCommands {
    /// Print the current working directory
    #[command()]
    Pwd,

    /// Change the local working directory
    #[command()]
    Cd,

    /// List the content of the given local directory
    #[command()]
    Dir,
}
