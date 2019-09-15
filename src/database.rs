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
        assert_eq!(create_table(), 0)    
    }
}
