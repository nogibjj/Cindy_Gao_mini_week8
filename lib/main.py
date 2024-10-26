import time
import psutil
from extract import extract
from transform_load import load
from query import read_query, update_query, delete_query, sorting_Change


def measure_performance(func, *args):
    """
    Measures the performance of a function by tracking time and memory usage.
    """
    process = psutil.Process()
    start_memory = process.memory_info().rss / 1024  # Memory in KB
    start_time = time.time()

    # Execute the function
    result = func(*args)

    # Get the performance metrics
    end_time = time.time()
    end_memory = process.memory_info().rss / 1024  # Memory in KB
    duration = end_time - start_time

    print(f"Execution Time: {duration * 1000:.2f}ms")  # Convert to ms
    print(f"Memory Usage Before: {start_memory:.2f} KB")
    print(f"Memory Usage After: {end_memory:.2f} KB")
    print(f"Memory Consumed: {end_memory - start_memory:.2f} KB\n")

    return result


def main():
    # Set the URL and file path
    url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/murder_2016/murder_2015_final.csv"
    file_path = "data/murder_2015_final.csv"

    # Measure performance of each function
    print("Performance of `extract` function:")
    measure_performance(extract, url, file_path)

    print("Performance of `load` function:")
    measure_performance(load, file_path)

    print("Performance of `read_query` function:")
    measure_performance(read_query)

    print("Performance of `update_query` function:")
    measure_performance(update_query)

    print("Performance of `delete_query` function:")
    measure_performance(delete_query)

    print("Performance of `sorting_change` function:")
    measure_performance(sorting_Change)


if __name__ == "__main__":
    main()
