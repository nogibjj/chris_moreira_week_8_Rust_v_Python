from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import query_create, query_read, query_update, query_delete

# Extract
extract()

# transform & load
load()

# Query
query_create()
query_read()
query_update()
query_delete()


def main_results():
    results = {
        "extract_to": extract(),
        "transform_db": load(),
        "create": query_create(),
        "read": query_read(),
        "update": query_update(),
        "delete": query_delete(),
    }
    return results
