use crate::funcs;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Index};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Task {
    pub title: Option<String>,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub completion: bool,
}
impl Task {
    pub fn new() -> Task {
        let new_task = Task {
            title: Some(funcs::return_string("Add a title to your new task")),
            description: Some(funcs::return_string("Add a Description to your new task")),
            due_date: Some(funcs::return_string("Add a due date to your new task")),
            completion: false,
        };
        return new_task;
    }
    pub fn delete(user_id: String, path: &str) {
        let title = funcs::return_string("What task would you like to delete?");
        let mut db_new = funcs::connect(path);
        let vec = db_new.data.get(&user_id).unwrap();
        let index = {
            let mut index: Option<usize> = None;
            for (i, n) in vec.iter().enumerate() {
                if let Some(title_compare) = n.title.clone() {
                    if title_compare == title {
                        index = Some(i);
                    } else {
                    }
                }
            }
            index
        };
        let mut db_clone = funcs::connect(path);
        let vec_clone = db_clone.data.get_mut(&user_id).unwrap();
        vec_clone.remove(index.unwrap());
        db_new.data.insert(user_id, vec_clone.to_vec());
        funcs::save(db_new, path).unwrap();
    }
    pub fn delete_auto(user_id: &String, path: &str, title: &String) {
        let mut db_new = funcs::connect(path);
        let vec = db_new.data.get(user_id).unwrap();
        let index = {
            let mut index: Option<usize> = None;
            for (i, n) in vec.iter().enumerate() {
                if let Some(title_compare) = n.title.clone() {
                    if &title_compare == title {
                        index = Some(i);
                    } else {
                    }
                }
            }
            index
        };
        let mut db_clone = funcs::connect(path);
        let vec_clone = db_clone.data.get_mut(user_id).unwrap();
        vec_clone.remove(index.unwrap());
        db_new.data.insert(user_id.clone(), vec_clone.to_vec());
        funcs::save(db_new, path).unwrap();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataBase {
    pub data: HashMap<String, Vec<Task>>,
}
impl DataBase {
    pub fn new() -> DataBase {
        let new_map = HashMap::new();
        let new_data = DataBase { data: new_map };
        return new_data;
    }
    pub fn insert_task(
        task: &Task,
        path: &str,
        mut db: DataBase,
        user_id: String,
    ) -> Result<(), std::io::Error> {
        let mut new_vec: Vec<Task> = db.data.get(&user_id).unwrap().to_vec();
        new_vec.push(task.clone());
        db.data.insert(user_id, new_vec);
        funcs::save(db, path).unwrap();
        Ok(())
    }
}

pub enum Errors {
    SaveError,
    SomeError,
    InvalidString,
}
