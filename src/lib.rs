#![doc = include_str!("../README.md")]


/// `macro_env!()` is used to fetch environment variables.
/// 
/// # Example
/// ```rust
/// // Import the crate, importing the whole crate is the easiest
/// // You can also manually import the function you need, for .env search for example:
/// // `use macro_env::dotenvreader;`
/// use macro_env::*;
/// 
/// // Fetch the environment variable "OS" from the .env file at the cargo.toml level
/// macro_env!(File, "OS");
///
/// // Fetch the environment variable "OS" from the system environment variables
/// macro_env!(System, "OS");
/// 
/// // Asks the user for enter the input through the terminal
/// macro_env!(Input);
/// 
/// // All, and not specifying the searchtype, will try to find the variable through all 3 methods:
/// // First it checks for a .env file
/// // Then by searching for a system variable
/// // And if both fail, it will ask the user for input
/// macro_env!(All, "OS");
/// macro_env!("OS");
///```
#[macro_export]
macro_rules! macro_env {
    (File, $envvariablename:literal) => {{
        dotenvreader($envvariablename.to_string()).unwrap()
    }};
    (System, $envvariablename:literal) => {
        systemreader($envvariablename.to_string()).unwrap()
    };
    (Input) => {{
        input().unwrap()
    }};
    (All, $envvariablename:literal) => {{
        let resultenv = dotenvreader($envvariablename.to_string());
        if resultenv.is_ok() {
            resultenv.unwrap()
        } else if systemreader($envvariablename.to_string()).is_ok() {
            systemreader($envvariablename.to_string()).unwrap()
        } else {
            input().unwrap()
        }
    }};
    ($envvariablename:literal) => {{
        let resultenv = dotenvreader($envvariablename.to_string());
        if resultenv.is_ok() {
            resultenv.unwrap()
        } else if systemreader($envvariablename.to_string()).is_ok() {
            systemreader($envvariablename.to_string()).unwrap()
        } else {
            input().unwrap()
        }
    }};
}

/// Reads the .env file and tries to find the .env variable.
/// 
/// # Example
/// ```rust
/// use macro_env::dotenvreader;
/// 
/// let envvariable :String = dotenvreader("OS".to_string()).unwrap();
/// ```
pub fn dotenvreader(envvariablename: String) -> Result<String, std::io::Error> {
    let file = std::fs::File::open(".env")?;
    let reader = std::io::BufReader::new(file);
    let mut token = String::new();
    use std::io::BufRead;

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 && parts[0] == envvariablename && !parts[1].is_empty() {
                token = parts[1].to_string();
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Couldn't find the variable requested in the .env",
                ));
            }
        }
    }

    if token.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "There doesn't seem to be a variable in the .env",
        ));
    }

    if token.ends_with('"') && token.starts_with('"') {
        token.pop();
        token.remove(0);
    };

    Ok(token)
}

/// Request user input
/// `input()` fetches stdin.read_lines() and then trims them.
/// 
/// # Example
/// ```rust
/// use macro_env::input;
/// 
/// // Request the user to input a variable
/// let envvariable :String = input().unwrap();
/// ```
pub fn input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    println!("Please enter an environment variable");
    std::io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();
    Ok(input)
}

/// Fetch the environment variable from the system environment variable
/// 
/// # Example
/// ```rust
/// use macro_env::systemreader;
/// 
/// // Using systemreader is just a shortcut for std::env::var()
/// let envvariable :String = systemreader("OS".to_string()).unwrap();
/// ```
pub fn systemreader(envvariablename: String) -> Result<String, std::env::VarError> {
    std::env::var(envvariablename)
}
/// Searchtype for the `fn envseeker()`, this will define what type of search it performs
pub enum SearchType {
    /// Searching for a .env file
    Envfile,
    /// Searching for a system variable
    System,
    /// Requesting user input
    Input,
    /// First searching for a .env file, then search for a system variable, and finally request the user to input one if all fails
    All,
}



/// A function instead of a macro to find the environment variable
/// 
/// # Example
/// ```rust
/// use macro_env::*;
/// use macro_env::SearchType::*;
/// 
/// // Fetch a variable from .env
/// let filevariable :String = envseeker(Envfile, "OS");
/// 
/// // Fetch a systemvariable
/// let systemvariable :String = envseeker(System, "OS");
/// 
/// // Request user input
/// let inputvariable :String = envseeker(Input, "OS");
/// 
/// // Perform all three methods to find a variable
/// let allvariable :String = envseeker(All, "OS"); 
/// ```
pub fn envseeker(searchtype: SearchType, envvariablename: &str) -> String {
    match searchtype {
        SearchType::System => systemreader(envvariablename.to_string()).unwrap(),
        SearchType::Envfile => dotenvreader(envvariablename.to_string()).unwrap(),
        SearchType::Input => input().unwrap(),
        SearchType::All => {
            let resultenv = dotenvreader(envvariablename.to_string());
            if resultenv.is_ok() {
                resultenv.unwrap()
            } else if systemreader(envvariablename.to_string().clone()).is_ok() {
                systemreader(envvariablename.to_string()).unwrap()
            } else {
                input().unwrap()
            }
        }
    }
}


#[cfg(feature = "typed")]
pub fn typedenv() {
    println!("Hello from the typed world");
}