use std::{
    fs::{self, File},
    iter,
    panic::UnwindSafe,
    path::PathBuf,
};

use crate::structs::{self, DataBase};

pub fn return_string(print: &str) -> String {
    println!("{}", print);
    let mut return_input = String::new();
    std::io::stdin()
        .read_line(&mut return_input)
        .expect("Invalid");
    return return_input.trim().to_string();
}

pub fn save(db: DataBase, path: &str) -> Result<(), std::io::Error> {
    let path_buf: PathBuf = PathBuf::from(path);
    fs::write(
        path_buf,
        serde_json::to_string_pretty::<structs::DataBase>(&db).unwrap(),
    )
}

pub fn return_u8(print: &str) -> u8 {
    println!("{}", print);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let new = input.trim().parse::<u8>().unwrap();
    return new;
}

pub fn serialize_db(path_buf: PathBuf) -> structs::DataBase {
    let serialized_db = fs::read_to_string(&path_buf).unwrap();
    let db: DataBase = serde_json::from_str(&serialized_db).unwrap();
    return db;
}

pub fn validate_user_id(user_id: &String, db: &structs::DataBase) -> bool {
    if db.data.get(user_id).is_some() {
        return true;
    } else {
        return false;
    }
}

fn new_task_vec() -> Vec<structs::Task> {
    let new_task = structs::Task::new();
    let mut new_vec: Vec<structs::Task> = Vec::new();
    new_vec.insert(0, new_task);
    return new_vec;
}

pub fn connect(path: &str) -> DataBase {
    let path_buf: PathBuf = PathBuf::from(path);

    if path_buf.exists() {
        return serialize_db(path_buf);
    } else {
        fs::create_dir_all(r"D:\rust_prova_final\database").unwrap();
        File::create(path).unwrap();
        fs::write(
            path,
            serde_json::to_string_pretty::<structs::DataBase>(&structs::DataBase::new()).unwrap(),
        )
        .unwrap();
        return serialize_db(path_buf);
    }
}

pub fn insert(path: &str, user_id: String) {
    let mut db_clone = connect(path);
    let mut insert_vec = new_task_vec();
    let vec = db_clone.data.get_mut(&user_id).unwrap();
    vec.append(&mut insert_vec);
    let mut db_new = connect(path);
    db_new.data.insert(user_id, vec.to_vec());
    save(db_new, path).unwrap();
}

pub fn edit(path: &str, user_id: String, mut task: Option<structs::Task>) {
    let mut db = connect(path);
    let checker = return_u8(
        "What would you like to edit? 
        [1] Title
        [2] Description
        [3] Due-Date
        [4] Mark as True
        [x] return
    ",
    );
    let title = String::from(task.as_mut().unwrap().title.clone().unwrap());
    pretty_print_task(path, user_id.clone(), task);
    let mut db_new = connect(path);
    let vec = db_new.data.get(&user_id).unwrap();
    let index = {
        let mut index: Option<usize> = None;
        for (i, n) in vec.iter().enumerate() {
            if let Some(title_compare) = n.title.clone() {
                if title_compare == title {
                    index = Some(i);
                } else {
                    index = Some(0);
                }
            }
        }
        index
    };

    let mut db_clone = connect(path);
    let vec_clone = db_clone.data.get_mut(&user_id).unwrap();
    let edit_task = vec_clone.get_mut(index.unwrap()).unwrap();

    match checker {
        1 => {
            edit_task.title = Some(return_string("New Title? "));
        }
        2 => {
            edit_task.description = Some(return_string("New Description? "));
        }
        3 => {
            edit_task.due_date = Some(return_string("New Date? "));
        }
        4 => {
            edit_task.completion ^= true;
        }
        _ => return,
    }
    let title = connect(path)
        .data
        .get(&user_id)
        .unwrap()
        .get(index.unwrap())
        .unwrap()
        .title
        .clone()
        .unwrap();
    structs::Task::delete_auto(&user_id, path, &title);
    let mut binding = connect(path);
    let vec_replaced = binding.data.get_mut(&user_id).unwrap();
    vec_replaced.insert(index.unwrap(), edit_task.to_owned());
    db.data.insert(user_id, vec_replaced.to_owned());
    save(db, path);
}

pub fn new_id(user_id: String, mut db: DataBase, path: &str) {
    db.data.insert(user_id, Vec::new());
    save(db, path).unwrap();
}

pub fn pretty_print_task_vec(tasks: &Vec<structs::Task>) {
    for task in tasks {
        println!(
            "({}) {:?}",
            bool_to_symbol(task.completion),
            task.title.as_ref().unwrap()
        );
    }
}

fn bool_to_symbol(bool: bool) -> String {
    if bool {
        return "V".to_string();
    } else {
        return "X".to_string();
    }
}

pub fn get_task(tasks_vec: &Vec<structs::Task>) -> Option<structs::Task> {
    let get_task = return_string("Select a task: (use title) ");
    for task in tasks_vec {
        if task.title.as_ref().unwrap() == &get_task {
            return Some(task.clone());
        } else {
            continue;
        }
    }
    println!("Value not found, Please try again...");
    None
}

pub fn get_task_auto(tasks_vec: &Vec<structs::Task>, taskid: String) -> Option<structs::Task> {
    for task in tasks_vec {
        if task.title.as_ref().unwrap() == &taskid {
            return Some(task.clone());
        } else {
            continue;
        }
    }
    println!("Value not found, Please try again...");
    None
}

pub fn pretty_print_task(path: &str, user_id: String, task: Option<structs::Task>) {
    let taskid = String::from(task.unwrap().title.unwrap());
    let task = get_task_auto(connect(path).data.get(&user_id).unwrap(), taskid);
    if task.is_some() {
        let task = task.unwrap();
        println!(
            "Title:  {:?}
    Description: {:?}
    Due Date:    {:?}
    Completion:  {:?}",
            unwrap_value_some(&task.title).unwrap_or(&"".to_string()),
            unwrap_value_some(&task.description).unwrap_or(&"".to_string()),
            unwrap_value_some(&task.due_date).unwrap_or(&"".to_string()),
            bool_to_symbol(task.completion)
        )
    } else {
        println!("Invalid Task! check for typos...")
    }
}

pub fn is_completed(completion: bool) -> String {
    if completion {
        return "Mark as Uncompleted".to_string();
    } else {
        return "Mark as Completed".to_string();
    }
}

fn unwrap_value_some<T>(option: &Option<T>) -> Result<&T, structs::Errors> {
    if let Some(value) = option {
        return Ok(value);
    } else {
        return Err(structs::Errors::SomeError);
    }
}
