pub fn create_table() -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_table()
    {
        assert_eq!(create_table(), 0)    
    }
}
