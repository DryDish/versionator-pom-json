use std::{env, fs::File, io::{BufRead, BufReader, Read, Write}, path::PathBuf};
use walkdir::WalkDir;
mod custom_error;
use custom_error::CustomError;

fn main() -> Result<(), CustomError> {
    let args: Vec<String> = env::args().collect();
    let working_dir = working_dir_params(args)?;
    let (source_file_name, source_search_word) = ("package.json", "\"version\"");
    let (target_file_name, target_search_word) = ("Pom.xml", "<version>");

    let (source_path, target_path) = get_file_paths(target_file_name, source_file_name, &working_dir)?;

    let source_file = File::open(source_path).expect("unable to open source file 😓");
    
    println!("Searching {} for the version.", &source_file_name);
    let version = version_from_package_json(source_search_word, source_file)?;
    println!("Version found: {}", version);

    let target_file = File::open(&target_path).expect("unable to open target file 😓");
    println!("Searching {} for the version tag to replace", &source_file_name);

    let fixed_version = replace_pom_version(target_search_word, &version, target_file)?;
    let mut target_file_write = File::create(target_path).expect("unable to open target file 😓");
    target_file_write.write_all(fixed_version.as_bytes()).expect("Failed to save to file 😱");
    
    println!("Main end");
    Ok(())
}

fn search_for_file(file_name: &str, path: &PathBuf) -> Result<PathBuf, CustomError> {
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        if entry.file_name().to_str().unwrap().contains(file_name) {
            return Ok(entry.into_path());
        }
    }
    Err(CustomError::FileNotFound)
}

fn get_file_paths(
    target_name: &str,
    source_name: &str,
    path: &PathBuf,
) -> Result<(PathBuf, PathBuf), CustomError> {
    let source_path_result = search_for_file(source_name, &path);
    let target_path_result = search_for_file(target_name, &path);
    let (source_path, target_path) = match (source_path_result, target_path_result) {
        (Ok(s_path), Ok(t_path)) => {
            println!("File '{}' found at {:?}", &source_name, &s_path);
            println!("File '{}' found at {:?}", &target_name, &t_path);
            (s_path, t_path)
        }
        (Err(s_err), Err(t_err)) => {
            println!(
                "Not able to find either file: {:?}, {:?}",
                &target_name, &source_name
            );
            println!("Errors:  {:?}, {:?}", &s_err, t_err);
            println!("Path searched: {:?}", &path);
            return Err(CustomError::FileNotFound);
        }
        (Ok(_), Err(t_err)) => {
            println!("Target path not found for: {:?}", &target_name);
            println!("Error: {:?}", &t_err);
            println!("Path searched: {:?}", &path);
            return Err(CustomError::TargetNotFound);
        }
        (Err(s_err), Ok(_)) => {
            println!("Source path not found: {:?}", &source_name);
            println!("Error: {:?}", &s_err);
            println!("Path searched: {:?}", &path);
            return Err(CustomError::SourceNotFound);
        }
    };
    return Ok((source_path, target_path));
}

fn working_dir_params(args: Vec<String>) -> Result<PathBuf, CustomError> {
    match args.len() {
        3 if args[1] == "-p" => return Ok(PathBuf::from(args[2].clone())),
        2 if args[1] == "-h" || args[1] == "-help" => {
            println!("HELP HAS ARRIVED!");
            println!("Call the binary on its own to use the calling directory as the search path.");
            println!("Call the binary with the -p flag to specify a directory to search.");
            println!("Example: <binary_name>");
            println!("Example: <binary_name> -p /home/my_user/java_project/");
            return Err(CustomError::BadParams);
        }
        1 => return Ok(env::current_dir()?),
        _ => {
            println!("I don't even know what you are trying to call 😂.");
            println!("Call the binary on its own to use the calling directory as the search path.");
            println!("Call the binary with the -p flag to specify a directory to search.");
            println!("Example: <binary_name>");
            println!("Example: <binary_name> -p /home/my_user/java_project/");
            return Err(CustomError::BadParams);
        }
    };
}

fn version_from_package_json(search_word: &str, file: File) -> Result<String, CustomError> {
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.contains(search_word) {
            // +4 to remove the ' : "' chars
            let start_index = line.find(search_word).unwrap() + search_word.len() + 3;
            let end_index = line.find(",").unwrap() -1;
            return Ok(line[start_index..end_index].to_string());
        }
    }
    return Err(CustomError::VersionNotFound);
}

fn replace_pom_version(search_word: &str, replacement_word: &str, file: File) -> Result<String, CustomError>{
    let mut counter = 0;
    let reader = BufReader::new(file);
    let mut return_string = String::new();
    for line in reader.lines() {
        let mut line = line?;
        if line.contains(search_word) {
            if counter == 1 {
                let start_index = line.find(search_word).unwrap() + search_word.len();
                let end_index = line.find("</").unwrap();
                println!("Pom.xml previous line |{}", &line);
                line.replace_range(start_index..end_index, replacement_word);
                println!("Pom.xml replaced line |{}", &line);
            }
            counter += 1;
        }
        return_string.push_str(&(line + &("\n".to_string())));
    }
    return Ok(return_string);
}
