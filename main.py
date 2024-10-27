import time
from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import query_create, query_read, query_update, query_delete

# Extract
extract()

# Transform & Load
load()

# Query
query_create()
query_read()
query_update()
query_delete()


def main_results():
    # Initialize results dictionary
    results = {
        "extract_to": extract(),
        "transform_db": load(),
    }

    # Time the query_create operation
    start_time = time.time()
    results["create"] = query_create()
    create_duration = time.time() - start_time
    print(f"query_create execution time: {create_duration:.4f} seconds")

    # Continue with other queries
    results["read"] = query_read()
    results["update"] = query_update()
    results["delete"] = query_delete()

    return results


if __name__ == "__main__":
    main_results()
