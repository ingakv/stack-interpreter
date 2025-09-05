mod test_length {
    use bprog::t;

    #[test]
    fn test_hello_length() {
        assert_eq!(t("\" hello \" length"), "5");
    }

    #[test]
    fn test_hello_world_length() {
        assert_eq!(t("\" hello world \" length"), "11");
    }

    #[test]
    fn test_list_length() {
        assert_eq!(t("[ 1 2 3 [ ] ] length"), "4");
    }

    #[test]
    fn test_block_length() {
        assert_eq!(t("{ 10 20 + } length"), "3");
    }
}