//! UCP Benchmark CLI
//!
//! Run LLM benchmarks and system performance tests.

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::info;
use tracing_subscriber::EnvFilter;
use ucp_bench::{
    fetch_openrouter_pricing,
    provider::{
        CerebrasProvider, GmiCloudProvider, GroqProvider, MockProvider, OpenRouterProvider,
    },
    runner::{BenchmarkConfig, BenchmarkRunner},
    test_cases::generate_test_cases,
};

#[derive(Parser)]
#[command(name = "ucp-bench")]
#[command(about = "UCP Benchmarking System for LLM evaluation")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run LLM benchmarks
    Llm {
        /// Provider to test (groq, cerebras, gmi, openrouter, mock)
        #[arg(short, long, default_value = "mock")]
        provider: String,

        /// Model to use
        #[arg(short, long)]
        model: Option<String>,

        /// Filter by command types (comma-separated)
        #[arg(short, long)]
        commands: Option<String>,

        /// Maximum concurrent requests
        #[arg(long, default_value = "3")]
        concurrency: usize,

        /// Output format (text, json, csv)
        #[arg(short, long, default_value = "text")]
        output: String,

        /// Save report to file
        #[arg(long)]
        save: Option<String>,

        /// Verbose output - show full prompts and responses for failures
        #[arg(short = 'v', long)]
        verbose: bool,

        /// Execute commands and validate document changes (not just parse validation)
        #[arg(long)]
        execute_commands: bool,
    },

    /// List available test cases
    List {
        /// Filter by command type
        #[arg(short, long)]
        command: Option<String>,
    },

    /// Show test document structure
    Document,

    /// Run system performance benchmarks
    System,

    /// Start the benchmark UI server
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "3001")]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if it exists
    if let Err(e) = dotenvy::dotenv() {
        // Only show error if .env exists but can't be loaded
        if std::path::Path::new(".env").exists() {
            eprintln!("Warning: Could not load .env file: {}", e);
        }
    }

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("ucp_bench=info".parse()?))
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Llm {
            provider,
            model,
            commands,
            concurrency,
            output,
            save,
            verbose,
            execute_commands,
        } => {
            run_llm_benchmark(
                provider,
                model,
                commands,
                concurrency,
                output,
                save,
                verbose,
                execute_commands,
            )
            .await?;
        }
        Commands::List { command } => {
            list_test_cases(command);
        }
        Commands::Document => {
            println!("{}", ucp_bench::test_document::document_description());
        }
        Commands::System => {
            println!("System benchmarks should be run with: cargo bench -p ucp-bench");
            println!("\nAvailable benchmarks:");
            println!("  - ID generation");
            println!("  - Content normalization");
            println!("  - Block operations");
            println!("  - Document validation");
            println!("  - UCL parsing");
        }
        Commands::Serve { port } => {
            info!("Starting benchmark UI server on port {}", port);
            ucp_bench::api::start_server(port).await?;
        }
    }

    Ok(())
}

async fn run_llm_benchmark(
    provider_name: String,
    model: Option<String>,
    commands: Option<String>,
    concurrency: usize,
    output: String,
    save: Option<String>,
    verbose: bool,
    execute_commands: bool,
) -> Result<()> {
    let mut config = BenchmarkConfig::new("llm-benchmark")
        .with_concurrency(concurrency)
        .with_execution(execute_commands);

    if execute_commands {
        info!("Full execution mode enabled - commands will be executed and document changes validated");
    }

    if let Some(cmds) = commands {
        let cmd_list: Vec<String> = cmds.split(',').map(|s| s.trim().to_uppercase()).collect();
        config = config.filter_commands(cmd_list);
    }

    let mut runner = BenchmarkRunner::new(config);

    // Add provider based on selection
    match provider_name.as_str() {
        "groq" => {
            let api_key =
                std::env::var("GROQ_API_KEY").expect("GROQ_API_KEY environment variable required");
            let model = model.unwrap_or_else(|| "llama-3.3-70b-versatile".into());
            info!("Using Groq provider with model: {}", model);
            runner = runner.add_provider(GroqProvider::new(api_key, model));
        }
        "cerebras" => {
            let api_key = std::env::var("CEREBRAS_API_KEY")
                .expect("CEREBRAS_API_KEY environment variable required");
            let model = model.unwrap_or_else(|| "llama-3.3-70b".into());
            info!("Using Cerebras provider with model: {}", model);
            runner = runner.add_provider(CerebrasProvider::new(api_key, model));
        }
        "gmi" => {
            let api_key =
                std::env::var("GMI_API_KEY").expect("GMI_API_KEY environment variable required");
            let model = model.unwrap_or_else(|| "deepseek-ai/DeepSeek-V3.1".into());
            info!("Using GMI Cloud provider with model: {}", model);
            runner = runner.add_provider(GmiCloudProvider::new(api_key, model));
        }
        "openrouter" => {
            let api_key = std::env::var("OPENROUTER_API_KEY")
                .expect("OPENROUTER_API_KEY environment variable required");
            // Fetch dynamic pricing from OpenRouter API
            info!("Fetching OpenRouter model pricing...");
            if let Err(e) = fetch_openrouter_pricing(&api_key).await {
                info!(
                    "Warning: Could not fetch OpenRouter pricing: {}. Using defaults.",
                    e
                );
            }
            let model = model.unwrap_or_else(|| "anthropic/claude-3.5-sonnet".into());
            info!("Using OpenRouter provider with model: {}", model);
            runner = runner.add_provider(OpenRouterProvider::new(api_key, model));
        }
        "mock" => {
            let model = model.unwrap_or_else(|| "mock-model".into());
            info!("Using Mock provider (no API calls)");
            runner = runner.add_provider(MockProvider::new(model));
        }
        "all" => {
            // Add all available providers
            if let Ok(key) = std::env::var("GROQ_API_KEY") {
                let model = "llama-3.3-70b-versatile".to_string();
                runner = runner.add_provider(GroqProvider::new(key, model));
            }
            if let Ok(key) = std::env::var("CEREBRAS_API_KEY") {
                let model = "llama-3.3-70b".to_string();
                runner = runner.add_provider(CerebrasProvider::new(key, model));
            }
            if let Ok(key) = std::env::var("GMI_API_KEY") {
                let model = "deepseek-ai/DeepSeek-V3.1".to_string();
                runner = runner.add_provider(GmiCloudProvider::new(key, model));
            }
            if let Ok(key) = std::env::var("OPENROUTER_API_KEY") {
                let _ = fetch_openrouter_pricing(&key).await;
                let model = "anthropic/claude-3.5-sonnet".to_string();
                runner = runner.add_provider(OpenRouterProvider::new(key, model));
            }
        }
        _ => {
            anyhow::bail!(
                "Unknown provider: {}. Use: groq, cerebras, gmi, openrouter, mock, or all",
                provider_name
            );
        }
    }

    info!("Starting benchmark...");
    let report = runner.run().await;

    // Output results
    match output.as_str() {
        "json" => {
            println!("{}", report.to_json()?);
        }
        "csv" => {
            println!("{}", report.to_csv());
        }
        _ => {
            report.print_summary();
        }
    }

    // Show verbose debug info for failures
    if verbose {
        println!("\n\n# Verbose Debug Output\n");
        for result in &report.results {
            if !result.is_success() {
                println!("## Failed Test: {} ({})\n", result.test_id, result.model);
                if let Some(ref prompt) = result.debug_prompt {
                    println!("### Full Prompt\n```\n{}\n```\n", prompt);
                }
                if let Some(ref raw) = result.debug_raw_response {
                    println!("### Raw LLM Response\n```\n{}\n```\n", raw);
                }
                println!("### Extracted UCL\n```\n{}\n```\n", result.generated_ucl);
                if let Some(ref err) = result.error_message {
                    println!("### Error\n{}\n", err);
                }
                println!("---\n");
            }
        }
    }

    // Save if requested
    if let Some(path) = save {
        if path.ends_with(".json") {
            report.save_json(&path)?;
        } else if path.ends_with(".csv") {
            report.save_csv(&path)?;
        } else {
            report.save_summary(&path)?;
        }
        info!("Report saved to: {}", path);
    }

    Ok(())
}

fn list_test_cases(filter_command: Option<String>) {
    let cases = generate_test_cases();

    println!("Available test cases:\n");
    println!("{:<30} {:<12} {}", "ID", "Command", "Description");
    println!("{}", "-".repeat(80));

    for case in cases {
        if let Some(ref cmd) = filter_command {
            if case.command_type.to_uppercase() != cmd.to_uppercase() {
                continue;
            }
        }
        println!(
            "{:<30} {:<12} {}",
            case.id, case.command_type, case.description
        );
    }
}
