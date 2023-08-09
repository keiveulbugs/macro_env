/// `macro_env!()` is used to fetch environment variables.
///
/// `macro_env!(File, "ExampleToken")` fetches a variable from the `.env` at the source folder with the name `ExampleToken`
///
/// `macro_env!(System, "ExampleToken")` fetches a variable from the systemvariables with the name `ExampleToken`
///
/// `macro_env!(Input)` requests the user for the environment variable at run time.
///
/// `macro_env!(All, "ExampleToken")` and `macro_env!("ExampleToken")` both perform all three options.
///
/// They first fetch the variable from `.env`, if not available it fetches from system variables, and if all fails it asks the user at run time.
#[macro_export]
macro_rules! macro_env {
    (File, $environmentname:literal) => {{
        envreader($environmentname.to_string()).unwrap()
    }};
    (System, $environmentname:literal) => {
        std::env::var($environmentname).unwrap()
    };
    (Input) => {{
        let mut input = String::new();
        println!("Please enter an environment variable");
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        input
    }};
    (All, $environmentname:literal) => {{
        let resultenv = envreader($environmentname.to_string());
        if resultenv.is_ok() {
            resultenv.unwrap()
        } else if std::env::var($environmentname).is_ok() {
            std::env::var($environmentname).unwrap()
        } else {
            let mut input = String::new();
            println!("Please enter an environment variable");
            std::io::stdin().read_line(&mut input).unwrap();
            input.trim().to_string()
        }
    }};
    ($environmentname:literal) => {{
        let resultenv = envreader($environmentname.to_string());
        if resultenv.is_ok() {
            resultenv.unwrap()
        } else if std::env::var($environmentname).is_ok() {
            std::env::var($environmentname).unwrap()
        } else {
            let mut input = String::new();
            println!("Please enter an environment variable");
            std::io::stdin().read_line(&mut input).unwrap();
            input.trim().to_string()
        }
    }};
}

pub fn envreader(environmentname: String) -> Result<String, std::io::Error> {
    let file = std::fs::File::open(".env").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut token = String::new();
    use std::io::BufRead;

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 && parts[0] == environmentname && !parts[1].is_empty() {
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
