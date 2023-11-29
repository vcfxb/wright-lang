
For many languages, threading can be a point of tension. When to use it (especially now that single-threaded async is more common),
how to use it, and how to optimize it are all common issues. 

In building wright, I decided it would be best to separate async and syncronous code/threads to avoid unnecessarily 
compiling/linking/running an async runtime to manage futures. 
