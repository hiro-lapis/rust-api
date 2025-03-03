### QA

Q. What benefit Rust gives us ?
A. Rust is the language can run fast and use right weight memory, which enables to cut down the cost in crowd environment.  Rust also has robust language features, such as ownership and strict type function. The robustness is enough to be used in business critical situation, like payment service.

Q. What is `into()`? 
A. into() is a trait that enables a struct to cast another type. If into is implemented, it implicitly enables from() as well.  

Q. Why main function fails when `cargo run`?  
A. `cargo run` simply execute rust file while `cargo make run` execute program following Makefile.toml which contains env info. Thus, `cargo make run` is dominant in practical application.  

Q. What is shared ?  
A. Common dependencies added by this project.  

Q. What is adapter?  
A. Adapter is the layer name that accesses persitance layers including repositories. repository, concreate implementation to connect DB and querying is one of the struct belongs adapter. `Adapter` means that the layer adapt the project api to external servers.  

Q. what api layer?
A. Api is the layer that receive input. handler mainly play the role in this functionality.  

Q. what api kernel?
A. Kernel is the layer that format and process input for following function. model handles domain logic in this layer. repository in this layer is interface for integration of  external services and regis. 

Q. Why repository exists in two layers, kernel and adapter?  
A. For testability, kernel/repository is helpful to mock functions that avoid executing external services.  

Q. What is workspace in rust?
A. Mudularization in rust is termed as workspace. Run `cargo new --lib`, writing workspace member in cargo.toml, and open crate `pub mod xxx` in lib.rs, enables create project's libraries.  

Q. 
A. 
