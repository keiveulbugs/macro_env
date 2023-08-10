Macro_env: An environment variable seeking crate
==================================================

Macro_env is a crate to find environment variables.  
Originally designed to easily fetch environment variables from different places without having to change a lot of different code.  
By simply chainging the SearchType in the macro or in the function, it fetches the variable from a different location.  


## Usage

First add:

```toml
[dependencies]
macro_env = "0.1.5"
```


**Macro**

```rust
// Import the crate, importing the whole crate is the easiest
// You can also manually import the function you need, for .env search for example:
// `use macro_env::dotenvreader;`
use macro_env::*;

// Fetch the environment variable "OS" from the .env file at the cargo.toml level
macro_env!(File, "OS");

// Fetch the environment variable "OS" from the system environment variables
macro_env!(System, "OS");

// Asks the user for enter the input through the terminal
macro_env!(Input);

// All, and not specifying the searchtype, will try to find the variable through all 3 methods:
// First it checks for a .env file
// Then by searching for a system variable
// And if both fail, it will ask the user for input
macro_env!(All, "OS");
macro_env!("OS");

```


**EnvSeeker()**

```rust
use macro_env::*;
use macro_env::SearchType::*;
// You can use envseeker() when you prefer using a function over a macro
envseeker(Envfile, "OS")
```

