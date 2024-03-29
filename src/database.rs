extern crate postgres;
use postgres::{Connection, TlsMode};

pub fn create_table() -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Person {
        id:   i32,
        name: String,
        data: Option<Vec<u8>>,
    }

    #[test]
    fn test_create_table()
    {
        let conn = Connection::connect(
                       "postgres://moomin@localhost:5432",
                       TlsMode::None
                   )
                  .unwrap();

        conn
            .execute(
                 "CREATE TABLE IF NOT EXISTS person (
                     id   SERIAL PRIMARY KEY,
                     name VARCHAR NOT NULL,
                     data BYTEA
                 )",
                 &[]
             )
            .unwrap();

        let me = Person {
                     id:   0,
                     name: "Steven".to_string(),
                     data: None,
                 };

        conn.execute(
                "INSERT INTO person (name, data) VALUES ($1, $2)",
                &[&me.name, &me.data]
             )
            .unwrap();

        for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
            let person = Person {
                             id:   row.get(0),
                             name: row.get(1),
                             data: row.get(2),
                         };
            println!("Found person {}: {}", person.id, person.name);
        }
    }
}
