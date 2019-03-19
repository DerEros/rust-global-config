# rust-global-config
Experiment on handling a global program configuration in a safe way

# Why?
Globals are evil. Rust - like most functional programming languages - tries to discourage use of global state. For good reasons. Unfortunately there are frameworks (AWS Lambda in my case) that take over control over the program flow and do not allow to pass in any previously allocated resources.

Use case: load configuration, create connection pools, do some expensive preparation. And then enter event loop (lambda!() macro in AWS' case).

Solution: have a global (yuk!) data structure that can be access by the code called by the framework.

Bonus objective: allow the initialization of the global data structure to be influenced by user input. This prevents the use of lazy_static initialization _only_. RwLock to the rescue!
