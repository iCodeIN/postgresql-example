use postgres::{Client, Error, NoTls};

fn main() -> Result<(), Error> {
    println!("\nConnecting to the database");
    let mut client = connect("user", "password", 5432, "postgres")?;

    println!("Creating table called `users`");
    create_users_table(&mut client)?;

    println!("\nPopulating `users`");
    populate_users_table(&mut client)?;
    print_from_query(&mut client)?;

    println!("\nUpdating username of the user with id `2`");
    update_user(&mut client)?;
    print_from_query(&mut client)?;

    println!("\nDelete user with id `1` and `3`");
    delete_user(&mut client)?;
    print_from_query(&mut client)?;

    Ok(())
}

/// Connect to a PostgreSQL instance.
fn connect(
    user: impl AsRef<str>,
    password: impl AsRef<str>,
    port: i32,
    database: impl AsRef<str>,
) -> Result<Client, Error> {
    let url = format!(
        "postgresql://{}:{}@localhost:{}/{}",
        user.as_ref(),
        password.as_ref(),
        port,
        database.as_ref(),
    );

    Client::connect(&url, NoTls)
}

/// Create a table named `users`.
fn create_users_table(client: &mut Client) -> Result<(), Error> {
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id          SERIAL PRIMARY KEY,
            username    VARCHAR UNIQUE NOT NULL,
            password    VARCHAR NOT NULL,
            email       VARCHAR UNIQUE NOT NULL
            )
    ",
    )
}

/// Populate the `users` table with 3 new users and their infos.
fn populate_users_table(client: &mut Client) -> Result<(), Error> {
    let query = "INSERT INTO users (username, password, email) VALUES ($1, $2, $3)";

    client.execute(query, &[&"user1", &"pass1", &"user1@test.com"])?;

    client.execute(query, &[&"user2", &"pass2", &"user2@test.com"])?;

    client.execute(query, &[&"user3", &"pass3", &"user3@test.com"])?;

    Ok(())
}

/// Update the username of the second user.
fn update_user(client: &mut Client) -> Result<(), Error> {
    client.execute(
        "UPDATE users SET username=$1 WHERE id=$2",
        &[&"jack1", &2_i32],
    )?;

    Ok(())
}

/// Delete all users but the one that has been modified.
fn delete_user(client: &mut Client) -> Result<(), Error> {
    let query = "DELETE FROM users WHERE id=$1";
    client.execute(query, &[&1_i32])?;
    client.execute(query, &[&3_i32])?;

    Ok(())
}

/// Print the content of the `users` table.
fn print_from_query(client: &mut Client) -> Result<(), Error> {
    for row in client.query("SELECT id, username, password, email FROM users", &[])? {
        let id: i32 = row.get(0);
        let username: &str = row.get(1);
        let password: &str = row.get(2);
        let email: &str = row.get(3);

        println!(
            "found user: {}) {} | {} | {}",
            id, username, password, email,
        );
    }

    Ok(())
}
