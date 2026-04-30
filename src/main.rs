mod models;
mod persistence;
pub mod views;

use crossterm::{cursor, execute, terminal};
use inquire::Select;
use persistence::context::DbContext;
use persistence::db;
use rusqlite::fallible_iterator::FallibleIterator;
use std::{fmt, io};
use crate::models::student::Student;
use crate::models::edit_new_action::EditNewAction;
use crate::models::main_menu_action::MainMenuAction;


fn ask_question(prompt: &str) -> String {
    let mut input = String::new();

    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

fn ask_confirm(prompt: &str) -> bool {
    loop {
        let input = ask_question(prompt).to_lowercase();

        match input.as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter 'y' or 'n'."),
        }
    }
}

fn clear() {
    let _ = execute!(
        io::stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = db::open("school.db")?;
    let mut ctx = DbContext::new(&conn);

    loop {
        clear();
        let actions = MainMenuAction::variants();

        let selection = Select::new("Select menu:", actions).prompt();

        let action = match selection {
            Ok(a) => a,
            Err(_) => {
                println!("Input cancelled. Exiting.");
                break;
            }
        };

        match action {
            MainMenuAction::Staff => {
                println!("Opening Staff Menu...");
                // call_staff_menu(&mut ctx);
            }
            MainMenuAction::Student => {
                println!("Opening Student Menu...");
                loop {
                    clear();

                    let students = ctx.students.fetch_all();
                    match students {
                        Ok(students) => {
                            println!("{}", "-".repeat(20));
                            for student in students {
                                println!(
                                    "{}: {}",
                                    student.student_id.unwrap(),
                                    student.full_name()
                                );
                                println!("Email: {}", student.student_email);
                                println!("{}", "-".repeat(20));
                            }
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                            break;
                        }
                    }

                    let actions = EditNewAction::variants();
                    let selection = Select::new("", actions).prompt();
                    let action = match selection {
                        Ok(a) => a,
                        Err(_) => {
                            println!("Input cancelled. Exiting.");
                            break;
                        }
                    };
                    match action {
                        EditNewAction::Edit => {
                            let student_id = ask_question("ID to edit: ").trim().parse::<i64>()?;
                            let student = ctx.students.fetch_by_id(student_id);
                            
                            // TODO: Implement edit
                            
                        }
                        EditNewAction::New => {
                            clear();
                            let last_name = ask_question("Last name: ");
                            let first_name = ask_question("First name: ");

                            let new_student = Student::new(first_name, last_name);
                            ctx.students.insert(&new_student).expect("TODO: panic message");
                        }
                        EditNewAction::Delete => {
                            let student_id = ask_question("ID to delete: ").trim().parse::<i64>()?;
                            let student = ctx.students.fetch_by_id(student_id);

                            if student.is_err() {
                                println!("Error fetching student: {}", student.err().unwrap());
                            } else {
                                println!("Student {} Selected: ", student?.full_name());
                                let confirm = ask_confirm("Are you sure? [y/n]");

                                if !confirm {
                                    println!("Aborting.");
                                    break;
                                } else {
                                    ctx.students.delete(student_id).expect("TODO: panic message");
                                }

                            }
                        }
                        EditNewAction::Back => {
                            break;
                        }
                    }
                }
            }
            MainMenuAction::Courses => {
                println!("Opening Courses Menu...");
                // call_courses_menu(&mut ctx);
            }
            MainMenuAction::TeacherLogin => {
                println!("Teacher login...");
                // handle_teacher_login(&mut ctx);
            }
            MainMenuAction::Quit => {
                println!("Goodbye!");
                break;
            }
        }
    }

    Ok(())
}
