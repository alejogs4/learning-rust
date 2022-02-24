use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

// Option 1
// fn read_file_lines(file_name: &str) -> Result<Vec<String>, io::Error> {
//     let file = File::open(file_name);
//     let mut file = match file {
//         Ok(f) => f,
//         Err(err) => match err.kind() {
//             io::ErrorKind::NotFound => match File::create(file_name) {
//                 Ok(created_file) => created_file,
//                 Err(err) => return Err(err),
//             },
//             _ => return Err(err),
//         },
//     };

//     let mut file_content = String::new();
//     match file.read_to_string(&mut file_content) {
//         Ok(_) => Ok(file_content
//             .lines()
//             .map(|str| String::from(str))
//             .collect::<Vec<String>>()),
//         Err(err) => Err(err),
//     }
// }

fn read_file_lines(file_name: &str) -> Result<Vec<String>, io::Error> {
    let mut file = File::open(file_name).or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            let file = File::create(file_name)?;
            return Ok(file);
        }
        Err(err)
    })?;

    let mut str_content = String::new();
    file.read_to_string(&mut str_content)?;

    let str_as_lines = str_content
        .lines()
        .filter(|str| str.len() > 0)
        .map(|str| String::from(str))
        .collect::<Vec<String>>();

    Ok(str_as_lines)
}

fn main() {
    let names = read_file_lines("names.txt");
    match names {
        Ok(names) => {
            for name in names {
                println!("{}", name);
            }
        }
        Err(err) => println!("{}", err.to_string()),
    }
}
