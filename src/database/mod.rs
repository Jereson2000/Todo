use sqlite::Connection;

pub(crate) fn add_tasks(task: String, conn: Connection) {
    let statement: String = format!("INSERT INTO tasks(task) VALUES('{task}');");
    conn.execute(statement).unwrap();
    println!("Added task: {task}");
}

pub(crate) fn list_tasks(conn: Connection) {
    let statement = String::from("SELECT * FROM tasks;");
    conn.iterate(statement, |row| {
        let collection: Vec<_> = row.iter().collect();
        println!(
            "{:?} - {:?}",
            collection[0].1.unwrap(),
            collection[1].1.unwrap()
        );
        true
    })
    .unwrap();
}

pub(crate) fn delete_tasks(id: Vec<u8>, conn: Connection) {
    for i in id {
        let statement = format!("DELETE FROM tasks WHERE id=={i};");
        conn.execute(statement).unwrap();
        println!("Deleted task with id: {i}")
    }
}

pub fn create_connection(path: String) -> Connection {
    // shadowing the path parameter
    let path = format!("{path}/tasks.db");
    let conn: Connection = sqlite::open(path).unwrap();
    let statement = String::from(
        "CREATE TABLE IF NOT EXISTS tasks(
            id INTEGER PRIMARY KEY,
            task TEXT);",
    );
    conn.execute(statement).unwrap();

    return conn;
}
