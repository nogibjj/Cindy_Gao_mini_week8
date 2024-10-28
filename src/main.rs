use clap::{Parser, Subcommand};
use cindy_gao_mini_week8::{delete_query, extract, load, read_query, sorting_change, update_query};
use std::error::Error;
use std::time::Instant;
use sysinfo::{System, SystemExt};

/// CLI application for managing the Murder2015 dataset with performance measurement
#[derive(Parser)]
#[command(name = "murder2015_cli")]
#[command(about = "A CLI for handling murder data in SQLite with performance tracking", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract data from a URL and save it to a file path
    Extract {
        url: String,
        file_path: String,
    },
    /// Load data from a CSV file into the SQLite database
    Load {
        dataset: String,
    },
    /// Read the top 5 entries from the database
    Read,
    /// Update a specific entry in the database
    Update,
    /// Delete a specific entry from the database
    Delete,
    /// Sort entries by change and display them
    Sort,
}

/// Measures the performance of a function by tracking time and memory usage.
fn measure_performance<F>(func: F) -> Result<(), Box<dyn Error>>
where
    F: Fn() -> Result<(), Box<dyn Error>>,
{
    let mut sys = System::new_all();
    sys.refresh_all();
    let start_memory = sys.used_memory();
    let start_time = Instant::now();

    // Execute the function
    func()?;

    // Refresh the system info to get updated memory usage
    sys.refresh_all();
    let end_memory = sys.used_memory();
    let duration = start_time.elapsed();

    println!("Execution Time: {:?}", duration);
    println!("Memory Usage Before: {} KB", start_memory);
    println!("Memory Usage After: {} KB", end_memory);
    println!(
        "Memory Consumed: {} KB",
        end_memory.saturating_sub(start_memory)
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Extract { url, file_path } => {
            measure_performance(|| extract(url, file_path))?;
        }
        Commands::Load { dataset } => {
            measure_performance(|| load(dataset))?;
        }
        Commands::Read => {
            measure_performance(read_query)?;
        }
        Commands::Update => {
            measure_performance(update_query)?;
        }
        Commands::Delete => {
            measure_performance(delete_query)?;
        }
        Commands::Sort => {
            measure_performance(sorting_change)?;
        }
    }
    Ok(())
}
