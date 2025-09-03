mod test_comparison {
    use bprog::t;

    #[test]
    fn test_less_than_operation() {
        assert_eq!(t("20 10 <"), "False");
    }

    #[test]
    fn test_greater_than_operation() {
        assert_eq!(t("20 10 >"), "True");
    }

    #[test]
    fn test_greater_than_operation_with_float() {
        assert_eq!(t("20 10.0 >"), "True");
    }

    #[test]
    fn test_float_greater_than_operation() {
        assert_eq!(t("20.0 20.0 >"), "False");
    }

    #[test]
    fn test_greater_than_or_equal_true() {
        assert_eq!(t("20 10 >="), "True");
    }

    #[test]
    fn test_greater_than_or_equal_false() {
        assert_eq!(t("10 20 >="), "False");
    }

    #[test]
    fn test_greater_than_or_equal_equal_values() {
        assert_eq!(t("10 10 >="), "True");
    }

    #[test]
    fn test_greater_than_or_equal_with_float() {
        assert_eq!(t("20 10.0 >="), "True");
    }

    #[test]
    fn test_greater_than_or_equal_int_float_equal() {
        use bprog::t;
        assert_eq!(t("10 10.0 >="), "True");
    }

    #[test]
    fn test_equality_operation() {
        assert_eq!(t("10 10 =="), "True");
    }

    #[test]
    fn test_equality_operation_with_float() {
        assert_eq!(t("10 10.0 =="), "True");
    }

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