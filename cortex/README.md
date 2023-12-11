# Cortex Schema Engine

The schema engine exposes a db agnostic/generic rust based API called `objects`  
which represent the usual `database` functionality.  
Those objects can be used to create a database schema from one of the `Cortex` db implementations  
this means that no matter what database you'll use as long as there is an underlying implementation  
for it you can create, update, modify the database or multiple databases as you please.  
This allows one to seamlessly migrate from database to database and update the schema as the product  
develops.

# Use Cases

- Exploring databases
  - Check performance between databases for your use cases
  - PoC
- Application creator is not in control of which database is being used (customer/user is in control of the database)
- Upgrade the schema as the product/application develops and changes for persistence might be needed.

# Behaviour

Based on the `objects` inside a `Step` `Vec<Statement>`
cortex schema engine handles the execution of database statements internally which means it obviously needs a connection
those statements are created through what is internally referred to as a `producer` a producer can produce something
in two ways `command` or `driver` a command as an example could be the creation of an SQL String
while a driver could be a direct call to the DB functionality through the DB protocol.
Not every database handles it's interface with text commands and or language based API wrapper.
When cortex executes the schema it'll either do it `transactional` or `optimistic` allowing for rollbacks if something goes wrong

# Exceptions

There are exceptions to the seamless transition from DB to DB and that is database specific functionality.
Unlike an ORM cortex does not want to abstract away database specific features therefor you might run into these cases.
