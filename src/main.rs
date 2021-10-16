use core::panic;
use std::{env, ffi::OsString, path::PathBuf, process::exit};
use walkdir::WalkDir;
mod custom_error;
use custom_error::CustomError;

fn main() -> Result<(), CustomError> {
    let args: Vec<String> = env::args().collect();
    let working_dir = working_dir_from_params(args)?;
    let source_name = "package.json";
    let target_name = "Pom.xml";

    let (source_path, target_path) = get_file_paths(target_name, source_name, &working_dir)?;

    

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

fn working_dir_from_params(args: Vec<String>) -> Result<PathBuf, CustomError>{
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
// use std::path::PathBuf;

// Ugly version: only current dir and 1 deep
// fn search_for_file(file_name: &str) -> Result<PathBuf, CustomError> {
//     let current_dir = env::current_dir().expect("Unable to get path to current dir");

//     println!("Searching {:?} for '{}'", &current_dir, file_name);

//     let folder_entries = fs::read_dir(current_dir)?;

//     for entry in folder_entries {
//         let entry = entry?;
//         if entry.path().ends_with(file_name) {
//             return Ok(entry.path());
//         }

//         if entry.file_type()?.is_dir() {
//             for sub_entry in fs::read_dir(entry.path())? {
//                 let sub_entry = sub_entry?;
//                 if sub_entry.path().ends_with(file_name) {
//                     return Ok(sub_entry.path());
//                 }
//             }
//         }
//     }
//     return Err(CustomError::FileNotFound);
// }
