use postgres::{Client, NoTls};

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
}

fn establish_connection() -> Result<Client, postgres::Error> {
    let connection_string = "host=postgres user=user password=mypassword dbname=crud_test_rust";
    let client = Client::connect(connection_string, NoTls)?;
    Ok(client)
}

fn create_user(client: &mut Client, name: &str, email: &str) -> Result<(), postgres::Error> {
    client.execute(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &[&name, &email],
    )?;
    Ok(())
}

fn read_users(client: &mut Client) -> Result<Vec<User>, postgres::Error> {
    let rows = client.query("SELECT id, name, email FROM users", &[])?;
    let users: Vec<User> = rows
        .iter()
        .map(|row| User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        })
        .collect();
    Ok(users)
}

fn update_user(client: &mut Client, id: i32, name: &str, email: &str) -> Result<(), postgres::Error> {
    client.execute(
        "UPDATE users SET name = $1, email = $2 WHERE id = $3",
        &[&name, &email, &id],
    )?;
    Ok(())
}

fn delete_user(client: &mut Client, id: i32) -> Result<(), postgres::Error> {
    client.execute("DELETE FROM users WHERE id = $1", &[&id])?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = establish_connection()?;

    // Create a user
    create_user(&mut client, "John Doe", "john@example.com")?;

    // Read all users
    let users = read_users(&mut client)?;
    for user in users {
        println!("User: {:?}", user);
    }

    // Update a user
    update_user(&mut client, 1, "John Smith", "john.smith@example.com")?;

    // Delete a user
    delete_user(&mut client, 1)?;

    Ok(())
}
