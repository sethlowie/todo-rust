use std::io;

struct Todo {
    title: String,
}

fn list_todos(todos: &Vec<Todo>) {
    println!("\n\nHere are your todos:\n");
    for t in todos {
        println!("{}", t.title);
    }
}

fn add_todo(todos: &mut Vec<Todo>, args: &[&str]) {
    if args.len() == 0 {
        println!("title is required");
    } else {
        todos.push(Todo {
            title: args.join(" ").to_string(),
        });
    }
}

fn main() {
    println!("TODO CLI APP!");

    let mut todos: Vec<Todo> = Vec::new();

    loop {
        println!(">");
        let mut cmd = String::new();

        io::stdin()
            .read_line(&mut cmd)
            .expect("Could not read line");

        let cmds = cmd.trim().split(" ").collect::<Vec<&str>>();

        let (head, tail) = cmds.split_at(1);

        match head[0] {
            "ls" => list_todos(&todos),
            "add" => add_todo(&mut todos, tail),
            _ => println!("Command {} not found", cmd.trim()),
        }
    }
}
