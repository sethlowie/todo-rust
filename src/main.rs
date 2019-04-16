use std::io;

struct Todo {
    title: String,
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
            "ls" => {
                println!("\nHERE ARE YOUR TODOS:");
                for (i, t) in todos.iter().enumerate() {
                    println!("{}", i + 1, t.title);
                }
            }
            "add" => {
                if tail.len() == 0 {
                    println!("title is required");
                } else {
                    todos.push(Todo {
                        title: tail.join(" ").to_string(),
                    });
                }
            }
            _ => println!("Command {} not found", cmd.trim()),
        }
    }
}
