struct Person {
    name: String,
    job: Option<String>
}

fn main() {
    let members = vec![
        Person {
            name: String::from("Joe"),
            job: Some(String::from("Manager")),
        },
        Person {
            name: String::from("Hary"),
            job: None,
        },
    ];

    for person in &members {
        match &person.job {
            Some(job) => println!("{} est {}.", person.name, job),
            None => println!("{} n'a pas de travail.", person.name)
        }
    }

}

