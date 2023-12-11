# Cortex Schema Engine

The schema engine exposes a db agnostic/generic rust based API called `objects`  
which represent the usual `database` functionality.  
Those objects can be used to create a database schema from one of the `Cortex` db implementations  
this means that no matter what database you'll use as long as there is an underlying implementation  
for it you can create, update, modify the database/s as you please.  
This allows one to seamlessly migrate from database to database and update the schema as the product  
develops.

# Use Cases

- Exploring databases
  - Check performance between databases
  - PoC
- Application creator is not in control of which database is being used (customer/user is in control of the database)
- Upgrade the schema as the product/application develops and changes for persistence might be needed.

# Exceptions

- Database exclusive features
