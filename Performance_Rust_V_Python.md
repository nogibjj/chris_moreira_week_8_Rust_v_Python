# Performance Improvements

This is a documentation that assesses the time performance improvements seen in rust when creating a table using sqlite. In this project, essnetially Python and RUst perform the same operations yield the same output, A database SpotifyDB.db. In that database, some simple CRUD query operations are performed: Create a table, Read a table, Update a table, Delete a row from that table. 

This assessment focuses solely on the "Create a table" function. Below is computed how long it took to perform this in Python Vs. Rust -- 

Here is how it takes to run the query_create fucntion with rust:
![alt text](image-3.png)

Here is how long it takes to run the query_create function with Python:
![alt text](image-2.png)


Python: The table creation took approximately 0.0065 seconds (or 6.5 milliseconds).
Rust: The table creation took approximately 725.3 microseconds (or 0.0007253 seconds).

Result: 
Rust is approximately 9 times faster than Python in creating the table in this particular instance.
