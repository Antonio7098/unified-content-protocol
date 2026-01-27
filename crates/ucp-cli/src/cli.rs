//! CLI definition and command dispatch

use clap::{Parser, Subcommand};

use crate::commands::{
    agent, block, document, edge, export, find, import, llm, nav, prune, snapshot, tree, tx, ucl,
    validate,
};
use crate::error::CliResult;

/// UCP - Unified Content Protocol CLI
///
/// A command-line tool for working with UCP documents, blocks, and content graphs.
#[derive(Parser)]
#[command(name = "ucp")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Enable trace-level logging
    #[arg(long, global = true)]
    pub trace: bool,

    /// Output format (text, json)
    #[arg(short, long, global = true, default_value = "text")]
    pub format: OutputFormat,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Clone, Copy, Debug, Default, clap::ValueEnum)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}

#[derive(Subcommand)]
pub enum Commands {
    // ===== Document Management =====
    /// Create a new UCP document
    Create {
        /// Output file path (prints to stdout if not specified)
        #[arg(short, long)]
        output: Option<String>,

        /// Document title
        #[arg(short, long)]
        title: Option<String>,
    },

    /// Display document information and statistics
    Info {
        /// Input file path (reads from stdin if not specified)
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Validate a document against the validation pipeline
    Validate {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Maximum allowed block count
        #[arg(long)]
        max_blocks: Option<usize>,

        /// Maximum nesting depth
        #[arg(long)]
        max_depth: Option<usize>,
    },

    // ===== Block Operations =====
    /// Block operations (add, get, delete, move, list, update)
    #[command(subcommand)]
    Block(BlockCommands),

    // ===== Edge Operations =====
    /// Edge (relationship) operations
    #[command(subcommand)]
    Edge(EdgeCommands),

    // ===== Navigation =====
    /// Navigate document structure
    #[command(subcommand)]
    Nav(NavCommands),

    /// Find blocks matching criteria
    Find {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Filter by semantic role
        #[arg(long)]
        role: Option<String>,

        /// Filter by tag
        #[arg(long)]
        tag: Option<String>,

        /// Search text pattern (regex)
        #[arg(long)]
        pattern: Option<String>,

        /// Maximum results
        #[arg(long, default_value = "50")]
        limit: usize,
    },

    /// Find orphaned (unreachable) blocks
    Orphans {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Display document hierarchy as a tree
    Tree {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Maximum depth to display
        #[arg(long)]
        depth: Option<usize>,

        /// Show block IDs
        #[arg(long)]
        ids: bool,
    },

    /// Prune orphaned or tagged blocks
    Prune {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,

        /// Only prune blocks with this tag
        #[arg(long)]
        tag: Option<String>,
    },

    // ===== Transactions =====
    /// Transaction operations
    #[command(subcommand)]
    Tx(TxCommands),

    // ===== Snapshots =====
    /// Snapshot (versioning) operations
    #[command(subcommand)]
    Snapshot(SnapshotCommands),

    // ===== Import/Export =====
    /// Import content from various formats
    #[command(subcommand)]
    Import(ImportCommands),

    /// Export document to various formats
    #[command(subcommand)]
    Export(ExportCommands),

    // ===== UCL Execution =====
    /// UCL (Unified Content Language) operations
    #[command(subcommand)]
    Ucl(UclCommands),

    // ===== Agent Traversal =====
    /// Agent traversal operations
    #[command(subcommand)]
    Agent(AgentCommands),

    // ===== LLM Integration =====
    /// LLM integration utilities
    #[command(subcommand)]
    Llm(LlmCommands),
}

// ===== Block Subcommands =====

#[derive(Subcommand)]
pub enum BlockCommands {
    /// Add a new block
    Add {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,

        /// Parent block ID (uses root if not specified)
        #[arg(short, long)]
        parent: Option<String>,

        /// Content type: text, markdown, code, json, table, math, media
        #[arg(short = 't', long, default_value = "text")]
        content_type: String,

        /// Block content (reads from stdin if not provided)
        #[arg(short, long)]
        content: Option<String>,

        /// Programming language (for code blocks)
        #[arg(long)]
        language: Option<String>,

        /// Block label
        #[arg(long)]
        label: Option<String>,

        /// Semantic role (e.g., title, heading1, body)
        #[arg(long)]
        role: Option<String>,

        /// Tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,
    },

    /// Get a block by ID
    Get {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Block ID
        id: String,

        /// Show only metadata
        #[arg(long)]
        metadata: bool,
    },

    /// Delete a block
    Delete {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,

        /// Block ID to delete
        id: String,

        /// Delete children recursively
        #[arg(long)]
        cascade: bool,

        /// Preserve children (move to parent)
        #[arg(long)]
        preserve_children: bool,
    },

    /// Move a block to a new location
    Move {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,

        /// Block ID to move
        id: String,

        /// New parent block ID
        #[arg(long)]
        to_parent: Option<String>,

        /// Move before this sibling
        #[arg(long)]
        before: Option<String>,

        /// Move after this sibling
        #[arg(long)]
        after: Option<String>,

        /// Index within new parent
        #[arg(long)]
        index: Option<usize>,
    },

    /// List all blocks in the document
    List {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Show only IDs
        #[arg(long)]
        ids_only: bool,
    },

    /// Update block content or metadata
    Update {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,

        /// Block ID
        id: String,

        /// New content
        #[arg(short, long)]
        content: Option<String>,

        /// New label
        #[arg(long)]
        label: Option<String>,

        /// New semantic role
        #[arg(long)]
        role: Option<String>,

        /// New summary
        #[arg(long)]
        summary: Option<String>,

        /// Add tag
        #[arg(long)]
        add_tag: Option<String>,

        /// Remove tag
        #[arg(long)]
        remove_tag: Option<String>,
    },
}

// ===== Edge Subcommands =====

#[derive(Subcommand)]
pub enum EdgeCommands {
    /// Add an edge between blocks
    Add {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,

        /// Source block ID
        source: String,

        /// Edge type (e.g., references, derived_from, supports)
        #[arg(short = 't', long)]
        edge_type: String,

        /// Target block ID
        target: String,

        /// Edge description
        #[arg(long)]
        description: Option<String>,

        /// Confidence score (0.0 - 1.0)
        #[arg(long)]
        confidence: Option<f64>,
    },

    /// Remove an edge
    Remove {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,

        /// Source block ID
        source: String,

        /// Edge type
        #[arg(short = 't', long)]
        edge_type: String,

        /// Target block ID
        target: String,
    },

    /// List edges for a block
    List {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Block ID
        id: String,

        /// Show only outgoing edges
        #[arg(long)]
        outgoing: bool,

        /// Show only incoming edges
        #[arg(long)]
        incoming: bool,
    },
}

// ===== Navigation Subcommands =====

#[derive(Subcommand)]
pub enum NavCommands {
    /// Show child blocks
    Children {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Block ID (uses root if not specified)
        id: Option<String>,
    },

    /// Show parent block
    Parent {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Block ID
        id: String,
    },

    /// Show sibling blocks
    Siblings {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Block ID
        id: String,
    },

    /// Show all descendants
    Descendants {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Block ID (uses root if not specified)
        id: Option<String>,

        /// Maximum depth
        #[arg(long)]
        depth: Option<usize>,
    },
}

// ===== Transaction Subcommands =====

#[derive(Subcommand)]
pub enum TxCommands {
    /// Begin a transaction
    Begin {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Output file path (with transaction state)
        #[arg(short, long)]
        output: Option<String>,

        /// Transaction name
        #[arg(long)]
        name: Option<String>,
    },

    /// Commit a transaction
    Commit {
        /// Input file path (with transaction state)
        #[arg(short, long)]
        input: Option<String>,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Rollback a transaction
    Rollback {
        /// Input file path (with transaction state)
        #[arg(short, long)]
        input: Option<String>,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Create a savepoint
    Savepoint {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,

        /// Savepoint name
        name: String,
    },
}

// ===== Snapshot Subcommands =====

#[derive(Subcommand)]
pub enum SnapshotCommands {
    /// Create a snapshot
    Create {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,

        /// Snapshot name
        name: String,

        /// Snapshot description
        #[arg(long)]
        description: Option<String>,
    },

    /// Restore from a snapshot
    Restore {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,

        /// Snapshot name
        name: String,
    },

    /// List all snapshots
    List {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Delete a snapshot
    Delete {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,

        /// Snapshot name
        name: String,
    },

    /// Compare two snapshots
    Diff {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// First snapshot name
        from: String,

        /// Second snapshot name
        to: String,
    },
}

// ===== Import Subcommands =====

#[derive(Subcommand)]
pub enum ImportCommands {
    /// Import from Markdown
    Markdown {
        /// Input Markdown file
        file: String,

        /// Output UCP document file
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Import from HTML
    Html {
        /// Input HTML file
        file: String,

        /// Output UCP document file
        #[arg(short, long)]
        output: Option<String>,

        /// Extract images
        #[arg(long)]
        extract_images: bool,

        /// Extract links
        #[arg(long)]
        extract_links: bool,
    },
}

// ===== Export Subcommands =====

#[derive(Subcommand)]
pub enum ExportCommands {
    /// Export to Markdown
    Markdown {
        /// Input UCP file
        #[arg(short, long)]
        input: Option<String>,

        /// Output Markdown file
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Export to JSON
    Json {
        /// Input UCP file
        #[arg(short, long)]
        input: Option<String>,

        /// Output JSON file
        #[arg(short, long)]
        output: Option<String>,

        /// Pretty print
        #[arg(long)]
        pretty: bool,
    },
}

// ===== UCL Subcommands =====

#[derive(Subcommand)]
pub enum UclCommands {
    /// Execute UCL commands
    Exec {
        /// Input document file
        #[arg(short, long)]
        input: Option<String>,

        /// Output document file
        #[arg(short, long)]
        output: Option<String>,

        /// UCL commands (reads from stdin if not provided)
        #[arg(short, long)]
        commands: Option<String>,

        /// UCL file to execute
        #[arg(short, long)]
        file: Option<String>,
    },

    /// Parse and validate UCL without executing
    Parse {
        /// UCL commands to parse
        #[arg(short, long)]
        commands: Option<String>,

        /// UCL file to parse
        #[arg(short, long)]
        file: Option<String>,
    },
}

// ===== Agent Subcommands =====

#[derive(Subcommand)]
pub enum AgentCommands {
    /// Session management
    #[command(subcommand)]
    Session(AgentSessionCommands),

    /// Navigate to a block
    Goto {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Session ID
        #[arg(short, long)]
        session: String,

        /// Target block ID
        target: String,
    },

    /// Go back in navigation history
    Back {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Session ID
        #[arg(short, long)]
        session: String,

        /// Number of steps to go back
        #[arg(default_value = "1")]
        steps: usize,
    },

    /// Expand from current position
    Expand {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Session ID
        #[arg(short, long)]
        session: String,

        /// Block ID (uses current position if not specified)
        id: Option<String>,

        /// Direction: down, up, both
        #[arg(long, default_value = "down")]
        direction: String,

        /// Expansion depth
        #[arg(long, default_value = "2")]
        depth: usize,
    },

    /// Follow an edge type
    Follow {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Session ID
        #[arg(short, long)]
        session: String,

        /// Edge type to follow
        edge_type: String,
    },

    /// Search for blocks
    Search {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Session ID
        #[arg(short, long)]
        session: String,

        /// Search query
        query: String,

        /// Maximum results
        #[arg(long, default_value = "10")]
        limit: usize,
    },

    /// Find blocks with conditions
    Find {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Session ID
        #[arg(short, long)]
        session: String,

        /// Filter by role
        #[arg(long)]
        role: Option<String>,

        /// Filter by tag
        #[arg(long)]
        tag: Option<String>,
    },

    /// Context window management
    #[command(subcommand)]
    Context(AgentContextCommands),

    /// View current position or block
    View {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Session ID
        #[arg(short, long)]
        session: String,

        /// View mode: full, preview, metadata, ids
        #[arg(long, default_value = "full")]
        mode: String,
    },
}

#[derive(Subcommand)]
pub enum AgentSessionCommands {
    /// Create a new agent session
    Create {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Session name
        #[arg(long)]
        name: Option<String>,

        /// Starting block ID
        #[arg(long)]
        start: Option<String>,
    },

    /// List active sessions
    List {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,
    },

    /// Close a session
    Close {
        /// Session ID
        session: String,
    },
}

#[derive(Subcommand)]
pub enum AgentContextCommands {
    /// Add blocks to context window
    Add {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Session ID
        #[arg(short, long)]
        session: String,

        /// Block IDs to add (comma-separated)
        ids: String,
    },

    /// Remove blocks from context window
    Remove {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Session ID
        #[arg(short, long)]
        session: String,

        /// Block IDs to remove (comma-separated)
        ids: String,
    },

    /// Clear context window
    Clear {
        /// Session ID
        #[arg(short, long)]
        session: String,
    },

    /// Show context window contents
    Show {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Session ID
        #[arg(short, long)]
        session: String,
    },
}

// ===== LLM Subcommands =====

#[derive(Subcommand)]
pub enum LlmCommands {
    /// Create ID mapping for token efficiency
    IdMap {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Output mapping file
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Convert UCL to use short IDs
    ShortenUcl {
        /// UCL commands
        ucl: String,

        /// Mapping file
        #[arg(short, long)]
        mapping: String,
    },

    /// Convert UCL from short to full IDs
    ExpandUcl {
        /// UCL with short IDs
        ucl: String,

        /// Mapping file
        #[arg(short, long)]
        mapping: String,
    },

    /// Generate prompt documentation for UCL capabilities
    Prompt {
        /// Capabilities to include (comma-separated or 'all')
        #[arg(long, default_value = "all")]
        capabilities: String,
    },

    /// Manage context window for LLM
    Context {
        /// Input file path
        #[arg(short, long)]
        input: Option<String>,

        /// Maximum tokens
        #[arg(long, default_value = "4000")]
        max_tokens: usize,

        /// Block IDs to include (comma-separated)
        #[arg(long)]
        blocks: Option<String>,
    },
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        match self.command {
            // Document Management
            Commands::Create { output, title } => document::create(output, title, self.format),
            Commands::Info { input } => document::info(input, self.format),
            Commands::Validate {
                input,
                max_blocks,
                max_depth,
            } => validate::validate(input, max_blocks, max_depth, self.format),

            // Block Operations
            Commands::Block(cmd) => block::handle(cmd, self.format),

            // Edge Operations
            Commands::Edge(cmd) => edge::handle(cmd, self.format),

            // Navigation
            Commands::Nav(cmd) => nav::handle(cmd, self.format),
            Commands::Find {
                input,
                role,
                tag,
                pattern,
                limit,
            } => find::find(input, role, tag, pattern, limit, self.format),
            Commands::Orphans { input } => find::orphans(input, self.format),
            Commands::Tree { input, depth, ids } => tree::tree(input, depth, ids, self.format),
            Commands::Prune { input, output, tag } => prune::prune(input, output, tag, self.format),

            // Transactions
            Commands::Tx(cmd) => tx::handle(cmd, self.format),

            // Snapshots
            Commands::Snapshot(cmd) => snapshot::handle(cmd, self.format),

            // Import/Export
            Commands::Import(cmd) => import::handle(cmd, self.format),
            Commands::Export(cmd) => export::handle(cmd, self.format),

            // UCL
            Commands::Ucl(cmd) => ucl::handle(cmd, self.format),

            // Agent
            Commands::Agent(cmd) => agent::handle(cmd, self.format),

            // LLM
            Commands::Llm(cmd) => llm::handle(cmd, self.format),
        }
    }
}
