use crate::funcs::{pretty_print_task, pretty_print_task_vec};

mod funcs;
mod structs;
const PATH: &'static str = r"D:\rust_prova_final\database\db.json";
fn main() {
    println!("Hello! (code is running...) ");
    loop {
        let user_id = funcs::return_string(
            "insert your user name to sign in, or type in a brand new one to create your login. (X to exit code)",
        );
        if user_id.trim() == "X".to_string() {
            std::process::exit(1);
        }
        let db = funcs::connect(PATH);
        if funcs::validate_user_id(&user_id, &db) {
            loop {
                let tasks_vec = db.data.get(&user_id).unwrap();
                let checker = funcs::return_u8(
                    "What would you like to do?  
                [1] Select a task
                [2] Create a new task 
                [3] Log Out",
                );
                if checker == 1 {
                    pretty_print_task_vec(tasks_vec);
                    let task: Option<structs::Task> = funcs::get_task(tasks_vec);
                    if task.is_none() {
                        break;
                    }
                    loop {
                        pretty_print_task(PATH, user_id.clone(), task.clone());
                        println!(
                            "
     Options:
    [1] Edit Task
    [2] Delete Task
    [3] Return"
                        );
                        let checker = funcs::return_u8("What would you like to do? ");
                        match checker {
                            1 => funcs::edit(PATH, user_id.clone(), task.clone()),
                            2 => structs::Task::delete(user_id.clone(), PATH),
                            3 => break,
                            _ => continue,
                        }
                        funcs::pretty_print_task(PATH, user_id.clone(), task.clone());
                    }
                } else if checker == 2 {
                    funcs::insert(PATH, user_id.clone());
                    continue;
                } else if checker == 3 {
                    break;
                }
            }
        } else {
            funcs::new_id(user_id.clone(), db, PATH);
            let u8 = funcs::return_u8(
                "New user id created!, would you like to create your first task? 
        [1] Yes
        [2] No",
            );
            if u8 == 1 {
                funcs::insert(PATH, user_id);
            } else if u8 == 2 {
                println!("Okay! Goodbye. ");
                break;
            } else {
                println!("Invalid Option... ")
            }
        }
    }
}
