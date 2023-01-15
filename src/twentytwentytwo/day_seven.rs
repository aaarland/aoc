use std::{error::Error, cell::RefCell};

//use Regex
use regex::Regex;
#[derive(Debug)]
#[derive(Clone)]
struct File {
    id: i32,
    name: String,
    size: i32,
    parent: i32
}

trait FileType {
    fn id(&self) -> i32;
    fn size(&self) -> i32;
    fn parent(&self) -> Option<i32> {
        None
    }
    fn files(&self) -> Option<Vec<File>> {
        None
    }

}

impl FileType for File {
    fn id(&self) -> i32 {
        self.id
    }

    fn size(&self) -> i32 {
        self.size
    }
    fn parent(&self) -> Option<i32> {
        Some(self.parent)
    }
}
#[derive(Debug)]

struct FileFolder {
    id: i32,
    name: String,
    parent: Option<i32>,
    files: Vec<File>,
    folders: Vec<FileFolder>,
}

impl FileType for FileFolder {
    fn id(&self) -> i32 {
        self.id
    }
    fn parent(&self) -> Option<i32> {
        self.parent
    }
    fn size(&self) -> i32 {
        let mut total_size = 0;
        for file in &self.files {
            total_size += file.size();
        }
        for folder in &self.folders {
            total_size += folder.size();
        }
        total_size
    }
    fn files(&self) -> Option<Vec<File>> {
        let mut all_files = Vec::new();
        for file in &self.files {
            all_files.push(file.clone());
        }
        for folder in &self.folders {
            if let Some(files) = folder.files() {
                for file in files {
                    all_files.push(file);
                }
            }
        }
        Some(all_files)
    }
}

impl FileFolder {
    fn new(name: String, id: i32, parent: Option<i32>) -> Self {
        Self {
            id,
            name,
            parent,
            files: Vec::new(),
            folders: Vec::new(),
            
        }
    }
    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
    fn add_folder(&mut self, folder: FileFolder) {
        self.folders.push(folder);
    }
}


struct FileSystem{
    folders: Vec<FileFolder>,
    files: Vec<File>
}

impl FileSystem {
    fn new(root_folder: FileFolder) -> Self {
        Self{folders: vec![root_folder], files: vec![]}
    }
    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
    fn add_folder(&mut self, folder: FileFolder) {
        self.folders.push(folder);
    }

    fn get_file(&self, id: i32) -> Option<&File> {
        for file in &self.files {
            if file.id() == id {
                return Some(file);
            }
        }
        None
    }

    fn get_folder(&mut self, id: i32) -> Option<&mut FileFolder> {
        let mut return_folder = None;
        self.folders.iter_mut().for_each(|folder| {
            if folder.id() == id {
                return_folder = Some(folder);
            }
        });
        return_folder
    }
}

pub fn solution(lines: Vec<String>) {
    //each line is a command
    //split the string by spaces
    let root_folder = FileFolder::new("/".to_string(), 0, None);
    let mut file_system = FileSystem::new(root_folder);
    let mut current_folder = 0;
    let mut id = 1;
    for line in lines {
        let mut split_line = line.split(" ");
        let command = split_line.next().unwrap();
        let argument = split_line.next().unwrap();
        //regex for number
        let number = Regex::new(r"[-+]?[0-9]+").unwrap();
        // regex for $
        let dollar = Regex::new(r"\$").unwrap();
        // regex for 'dir'
        let dir = Regex::new(r"dir").unwrap();

        if number.is_match(command ){
            let new_file = File {
                id,
                name: argument.to_string(),
                size: command.parse::<i32>().unwrap(),
                parent: current_folder
            };
            file_system.add_file(new_file);
        }else if dollar.is_match(command){
            match argument {
                "cd" => {
                    let next_folder = split_line.next().unwrap();
                    println!("cd {}" , &next_folder);
                    match next_folder {
                        ".." => {
                            current_folder = file_system.get_file(current_folder).unwrap().parent().unwrap();
                        },
                        _ => {
                            current_folder = file_system.get_file(current_folder).unwrap().id();
                        }
                    
                    }

                },
                "ls" => {
                    println!("ls {}" , command);
                    for file in file_system.get_file(current_folder).unwrap().files().unwrap() {
                        println!("{} : {}", file.size, file.name);
                    }
                },
                _ => {
                    println!("Invalid command");
                }
                
            }
        }else if dir.is_match(command){
            let new_folder = FileFolder::new(argument.to_string(), id, Some(current_folder));
            file_system.add_folder(new_folder);
            file_system.get_folder(current_folder).unwrap().add_folder(&new_folder);
        }else{
            //do something
            println!("Invalid command");
        }
        id += 1;
    }
}
