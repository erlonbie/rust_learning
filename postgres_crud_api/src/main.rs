use std::io;
use std::error::Error;
use postgres::{Connection, TlsMode};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}
fn create_person(conn: &Connection, name: &str, age: i32) -> Result<(), Box<dyn Error>> {

    let stmt = conn.prepare("INSERT INTO person (name, age) VALUES ($1, $2)")?;
    let _ = stmt.execute(&[&name, &age])?;
    Ok(())
}

fn read_person(conn: &Connection, id: i32) -> Result<Person, Box<dyn Error>> {
    let stmt = conn.prepare("SELECT id, name, age FROM person WHERE id = $1")?;
    let rows = stmt.query(&[&id])?;
    let row = rows.get(0);
    Ok(Person {
        id: row.get(0),
        name: row.get(1),
        age: row.get(2),
    })
}

fn update_person(conn: &Connection, id: i32, name: &str, age: i32) -> Result<(), Box<dyn Error>> {
    let stmt = conn.prepare("UPDATE person SET name = $1, age = $2 WHERE id = $3")?;
    let _ = stmt.execute(&[&name, &age, &id])?;
    Ok(())
}

fn delete_person(conn: &Connection, id: i32) -> Result<(), Box<dyn Error>> {
    let stmt = conn.prepare("DELETE FROM person WHERE id = $1")?;
    let _ = stmt.execute(&[&id])?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::connect("postgres://username:password@localhost:5432/dbname", TlsMode::None)?;
    create_person(&conn, "John Doe", 30)?;
    let person = read_person(&conn, 1)?;
    println!("{:?}", person);
    update_person(&conn, 1, "Jane Doe", 35)?;
    let updated_person = read_person(&conn, 1)?;
    println!("{:?}", updated_person);
    delete_person(&conn, 1)?;
    Ok(())
}
