mod test_comparison {
    use stack_interpreter::t;
    

    #[test]
    fn test_boolean_equality_operation() {
        assert_eq!(t("True True =="), "True");
    }

    #[test]
    fn test_nested_equality_operation() {
        assert_eq!(t("True 40 40 == =="), "True");
    }

    #[test]
    fn test_string_equality_operation() {
        assert_eq!(t("\" abba \" \" abba \" =="), "True");
    }

    #[test]
    fn test_empty_list_equality_operation() {
        assert_eq!(t("[ ] [ ] =="), "True");
    }

    #[test]
    fn test_list_equality_operation() {
        assert_eq!(t("[ 1 2 ] [ 1 2 ] =="), "True");
    }

    #[test]
    fn test_nested_list_equality_operation() {
        assert_eq!(t(" [ [ ] ] [ [ ] ] =="), "True");
    }
}