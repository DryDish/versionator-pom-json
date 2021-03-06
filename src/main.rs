use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};
mod custom_error;
use custom_error::CustomError;

// Ideal usage: <binary> -s /source/path -t /target/path -n index number
const APP_NAME: &str = "versionator";

fn main() -> Result<(), CustomError> {
    let args: Vec<String> = env::args().collect();
    let (source, target, version_instance_count) = match handle_params(args) {
        Ok(tuple) => tuple,
        Err(error) => return Err(error),
    };
    let search_word = "version";
    let match_word = "<version>";
    let source_file = match File::open(&source) {
        Ok(file) => file,
        Err(_) => {
            println!(
                "Path does not point to a package.json | Path: {:?}",
                &source
            );
            return Err(CustomError::FileNotFound);
        }
    };
    let target_file = match File::open(&target) {
        Ok(file) => file,
        Err(_) => {
            println!("Path does not point to a pom.xml | Path: {:?}", &target);
            return Err(CustomError::FileNotFound);
        }
    };
    let version = match version_from_package_json(&search_word, &source_file) {
        Ok(version) => version,
        Err(error) => {
            println!(
                "Unable to find '{}' in package.json at: {:?}",
                &search_word, source_file
            );
            return Err(error);
        }
    };
    println!("Version read from package.json: '{}'", &version);

    let fixed_version = match replace_pom_version(
        &match_word,
        &version,
        target_file,
        version_instance_count.clone(),
    ) {
        Ok(changed) => changed,
        Err(error) => {
            return Err(error);
        }
    };
    let mut target_file_write = match File::create(&target) {
        Ok(file) => file,
        Err(_) => {
            println!("Unable to write to file at path: {:?}", target);
            return Err(CustomError::IoError);
        }
    };
    match target_file_write.write_all(fixed_version.as_bytes()) {
        Ok(_) => {
            println!(
                "Version {} placed on the {} instance of {} in the pom.xml..",
                &version, &version_instance_count, match_word
            );
            return Ok(());
        }
        Err(_) => {
            println!("Unable to write to file at path: {:?}", target);
            return Err(CustomError::IoError);
        }
    }
}

fn handle_params(args: Vec<String>) -> Result<(PathBuf, PathBuf, u8), CustomError> {
    match args.len() {
        //6 if args[1] == "-s" && args[3] == "-t" && args[4] == "-p" => {
        4 => {
            let number = match args[3].parse::<u8>() {
                Ok(number) => number,
                Err(_) => {
                    println!("Please provide a valid version instance number (0-255)");
                    return Err(CustomError::BadParams);
                }
            };
            return Ok((
                PathBuf::from(args[1].clone()),
                PathBuf::from(args[2].clone()),
                number,
            ));
        }
        2 if args[1] == "-h" || args[1] == "--help" || args[1] == "-H" => {
            print_help_text();
            return Err(CustomError::HelpPrinted);
        }
        _ => {
            print_help_text();
            return Err(CustomError::BadParams);
        }
    };
}

fn print_help_text() {
    println!("-----------------------------HELP-HAS-ARRIVED------------------------------------");
    println!();
    println!("This binary is designed to take the version number from your package.json");
    println!("and replace the version in your pom.xml file. Since pom.xml has multiple version");
    println!("tags, provide the tag number you would like replaced. The app counts instances of");
    println!("tags with the pattern '<version>' and replaces the one you passed.");
    println!("version tag number is any number from 0 to 255");
    println!();
    println!("Usage:");
    println!(
        "      {} [package.json path] [pom.xml path] [version tag number]",
        APP_NAME
    );
    println!();
    println!("Example:");
    println!(
        "      {} /path/to/package.json /path/to/pom.xml <number>",
        APP_NAME
    );
    println!();

    println!("---------------------------------------------------------------------------------");
}

fn version_from_package_json(search_word: &str, file: &File) -> Result<String, CustomError> {
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.contains(search_word) {
            // +4 to remove the ' : "' chars
            let start_index = line.find(search_word).unwrap() + search_word.len() + 4;
            let end_index = line.find(",").unwrap() - 1;
            return Ok(line[start_index..end_index].to_string());
        }
    }
    return Err(CustomError::VersionNotFound);
}

fn replace_pom_version(
    search_word: &str,
    replacement_string: &str,
    file: File,
    search_word_count: u8,
) -> Result<String, CustomError> {
    let mut counter = 0;
    let mut match_found = false;
    let reader = BufReader::new(file);
    let mut return_string = String::new();
    for line in reader.lines() {
        let mut line = line?;
        if line.contains(search_word) {
            if counter == search_word_count {
                let start_index = line.find(search_word).unwrap() + search_word.len();
                let end_index = line.find("</").unwrap();
                println!("Pom.xml previous line |{}", &line);
                line.replace_range(start_index..end_index, replacement_string);
                println!("Pom.xml replaced line |{}", &line);
                match_found = true;
            }
            counter += 1;
        }
        return_string.push_str(&(line + &("\n".to_string())));
    }
    if match_found {
        return Ok(return_string);
    } else {
        println!(
            "Version tag instance {} not found in pom.xml",
            &search_word_count
        );
        return Err(CustomError::VersionNotFound);
    }
}
