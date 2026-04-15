use zero_copy_env::{split_kv, parse, get, dotenv};

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

#[test]
fn test_env_real() {
    let path = get("PATH");
    assert!(path.is_some());
}

#[test]
fn test_dotenv() {
    let data = "A=1\nB=2\n";
    let mut it = dotenv::parse(data);

    assert_eq!(it.next(), Some(("A", "1")));

    assert!(matches!(
        it.next(),
        Some(("B", "1")) | Some(("B", "2"))
    ));
}

#[test]
fn test_existing_var() {
    let v = get("PATH");
    assert!(v.is_some());
}

#[test]
fn test_missing_var() {
    let v = get("THIS_VAR_DOES_NOT_EXIST_12345");
    assert!(v.is_none());
}

#[test]
fn test_unicode_key_handling() {
    let (k, v) = split_kv("UNICODE_Á=value").unwrap();

    assert_eq!(k, "UNICODE_Á");
    assert_eq!(v, "value");
}

#[test]
fn test_empty_env_key() {
    let v = get("");
    assert!(v.is_none());
}