use cindy_gao_mini_week8::{delete_query, extract, load, read_query, sorting_change, update_query};
use std::error::Error;
use std::time::Instant;
use sysinfo::{System, SystemExt};

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
    let url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/murder_2016/murder_2015_final.csv";
    let file_path = "data/murder_2015_final.csv";

    // Measure performance of each function
    measure_performance(|| extract(url, file_path))?;
    measure_performance(|| load(file_path))?;
    measure_performance(read_query)?;
    measure_performance(update_query)?;
    measure_performance(delete_query)?;
    measure_performance(sorting_change)?;

    Ok(())
}
