#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let (k, v) = split_kv("A=B").unwrap();
        assert_eq!(k, "A");
        assert_eq!(v, "B");
    }

    #[test]
    fn test_parse() {
        let v: u16 = parse("123").unwrap();
        assert_eq!(v, 123);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_env_real() {
        let path = get("PATH");
        assert!(path.is_some());
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_dotenv() {
        let data = "A=1\nB=2\n";
        let mut it = dotenv::parse(data);

        assert_eq!(it.next(), Some(("A", "1")));
        assert_eq!(it.next(), Some(("B", "2")));
    }

    #[test]
    fn compare_with_std() {
        let a = std::env::var("PATH").ok();
        let b = zero_copy_env::get("PATH");

        assert_eq!(a.as_deref(), b);
    }

    #[test]
    fn test_existing_var() {
        let v = zero_copy_env::get("PATH");
        assert!(v.is_some());
    }

    #[test]
    fn test_missing_var() {
        let v = zero_copy_env::get("THIS_VAR_DOES_NOT_EXIST_12345");
        assert!(v.is_none());
    }

    #[test]
    fn test_unicode_key_handling() {
        std::env::set_var("UNICODE_Á", "value");

        let v = zero_copy_env::get("UNICODE_Á");
        assert!(v.is_some());
    }

    #[test]
    fn test_empty_env_key() {
        let v = zero_copy_env::get("");
        assert!(v.is_none());
    }
}
