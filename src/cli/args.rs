//! Command line interface argument definitions via clap
use clap::{Parser, Subcommand};

fn parse_concurrency(s: &str) -> Result<usize, String> {
    let v: usize = s.parse().map_err(|_| format!("invalid integer: {}", s))?;
    if (1..=10000).contains(&v) {
        Ok(v)
    } else {
        Err(format!("{} is not in 1..=10000", v))
    }
}

fn parse_retries(s: &str) -> Result<usize, String> {
    let v: usize = s.parse().map_err(|_| format!("invalid integer: {}", s))?;
    if v <= 10 {
        Ok(v)
    } else {
        Err(format!("{} is not in 0..=10", v))
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "ampscan",
    about = "AmpScan — DDoS Amplification Port Testing Tool",
    long_about = "Tests DDoS amplification ports across IPv4/IPv6 prefixes.\n\
                  Requires AMPSCAN_DB_KEY as an environment variable for database encryption.",
    version
)]
pub struct Cli {
    /// Database file path (required for database management and scans without --prefix)
    #[arg(long, global = true, env = "AMPSCAN_DB_PATH")]
    pub db_path: Option<String>,

    /// Admin user (or set AMPSCAN_USER environment variable)
    #[arg(short, long, global = true, env = "AMPSCAN_USER")]
    pub user: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize database and create administrator user
    Init,

    /// Manage amplification ports
    #[command(subcommand)]
    Port(PortCommands),

    /// Manage network prefixes (CIDR)
    #[command(subcommand)]
    Prefix(PrefixCommands),

    /// Manage administrator users
    #[command(subcommand)]
    User(UserCommands),

    /// Execute amplification scan
    #[command(subcommand)]
    Scan(ScanCommands),

    /// Launch interactive Terminal User Interface (TUI)
    Tui {
        /// Maximum number of simultaneous probes (1-10000)
        #[arg(short, long, default_value = "256", value_parser = parse_concurrency)]
        concurrency: usize,
        /// Timeout per probe in seconds (1-60)
        #[arg(short, long, default_value = "3", value_parser = clap::value_parser!(u64).range(1..=60))]
        timeout: u64,
        /// Number of retries for UDP probes (0-10, default is 2)
        #[arg(short, long, default_value = "2", value_parser = parse_retries)]
        retries: usize,
        /// Request PDF report generation when scan finishes
        #[arg(long)]
        pdf: bool,
        /// Output PDF report file path
        #[arg(short, long, default_value = "ampscan_report.pdf")]
        output: String,
        /// Company/client name for the report
        #[arg(long)]
        client_name: Option<String>,
        /// PDF report recipient (e.g., manager or department name)
        #[arg(long)]
        recipient: Option<String>,
        /// Audit HTTP (80) and HTTPS (443) for PHP RCE (PHP2Shell / phpAfterShell)
        #[arg(long)]
        check_php: bool,
        /// Custom Host header and TLS SNI for --check-php probes (e.g. example.com)
        #[arg(long)]
        http_host: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum PortCommands {
    /// List all registered ports
    List,
    /// Enable a port for testing
    Enable {
        /// Port ID
        id: i64,
    },
    /// Disable a port (will not be tested)
    Disable {
        /// Port ID
        id: i64,
    },
}

#[derive(Subcommand, Debug)]
pub enum PrefixCommands {
    /// List all registered prefixes
    List,
    /// Add a new network prefix
    Add {
        /// CIDR Prefix (e.g., 192.168.0.0/24 or 2001:db8::/120)
        #[arg(short, long)]
        prefix: String,
        /// Description (e.g., client name, AS)
        #[arg(short, long, default_value = "")]
        description: String,
    },
    /// Edit an existing prefix
    Edit {
        /// Prefix ID
        id: i64,
        /// New CIDR Prefix
        #[arg(short, long)]
        prefix: Option<String>,
        /// New description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Remove a prefix
    Remove {
        /// Prefix ID
        id: i64,
    },
    /// Enable a prefix for scanning
    Enable {
        /// Prefix ID
        id: i64,
    },
    /// Disable a prefix (will not be scanned)
    Disable {
        /// Prefix ID
        id: i64,
    },
}

#[derive(Subcommand, Debug)]
pub enum UserCommands {
    /// List users
    List,
    /// Add a new administrator user
    Add {
        /// Username
        #[arg(short = 'n', long)]
        username: String,
    },
    /// Change a user's password
    ChangePassword {
        /// Username
        #[arg(short = 'n', long)]
        username: String,
    },
    /// Remove a user
    Remove {
        /// User ID
        id: i64,
    },
}

#[derive(Subcommand, Debug)]
pub enum ScanCommands {
    /// Run a full scan on all enabled prefixes
    Run {
        /// Maximum number of simultaneous probes (1-10000)
        #[arg(short, long, default_value = "256", value_parser = parse_concurrency)]
        concurrency: usize,
        /// Timeout per probe in seconds (1-60)
        #[arg(short, long, default_value = "3", value_parser = clap::value_parser!(u64).range(1..=60))]
        timeout: u64,
        /// Number of retries for UDP probes (0-10, default is 2, giving 3 total attempts)
        #[arg(short, long, default_value = "2", value_parser = parse_retries)]
        retries: usize,
        /// Output PDF report file path
        #[arg(short, long, default_value = "ampscan_report.pdf")]
        output: String,
        /// Manual CIDR prefix to test directly (ignores database and skips PDF)
        #[arg(long)]
        prefix: Option<String>,
        /// Request PDF report generation
        #[arg(long)]
        pdf: bool,
        /// Company/client name for the report
        #[arg(long)]
        client_name: Option<String>,
        /// PDF report recipient (e.g., manager or department name)
        #[arg(long)]
        recipient: Option<String>,
        /// Audit HTTP (80) and HTTPS (443) for PHP RCE (PHP2Shell / phpAfterShell)
        #[arg(long)]
        check_php: bool,
        /// Custom Host header and TLS SNI for --check-php probes (e.g. example.com)
        #[arg(long)]
        http_host: Option<String>,
    },
    /// Test a single IP against all enabled ports
    Single {
        /// IP Address (IPv4 or IPv6)
        ip: String,
        /// Timeout per probe in seconds (1-60)
        #[arg(short, long, default_value = "3", value_parser = clap::value_parser!(u64).range(1..=60))]
        timeout: u64,
        /// Maximum number of simultaneous probes (1-10000)
        #[arg(short, long, default_value = "1", value_parser = parse_concurrency)]
        concurrency: usize,
        /// Number of retries for UDP probes (0-10, default is 2, giving 3 total attempts)
        #[arg(short, long, default_value = "2", value_parser = parse_retries)]
        retries: usize,
        /// Audit HTTP (80) and HTTPS (443) for PHP RCE (PHP2Shell / phpAfterShell)
        #[arg(long)]
        check_php: bool,
        /// Custom Host header and TLS SNI for --check-php probes (e.g. example.com)
        #[arg(long)]
        http_host: Option<String>,
    },
}
