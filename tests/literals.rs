mod test_literals {
    use stack_interpreter::t;

    #[test]
    fn test_literal_int() {
        assert_eq!(t("5"), "5")
    }

    #[test]
    fn test_literal_long_int() {
        assert_eq!(
            t("121231324135634563456363567"),
            "121231324135634563456363567"
        )
    }

    #[test]
    fn test_literal_float() {
        assert_eq!(t("1.0"), "1.0")
    }

    #[test]
    fn test_literal_float_zero() {
        assert_eq!(t("0.0"), "0.0")
    }

    #[test]
    fn test_literal_negative_int() {
        assert_eq!(t("-1"), "-1")
    }

    #[test]
    fn test_literal_negative_float() {
        assert_eq!(t("-1.1"), "-1.1")
    }

    #[test]
    fn test_literal_bool_false() {
        assert_eq!(t("False"), "False")
    }

    #[test]
    fn test_literal_bool_true() {
        assert_eq!(t("True"), "True")
    }

    #[test]
    fn test_literal_nested_list() {
        assert_eq!(t("[ [ ] [ ] ]"), "[[],[]]")
    }

    #[test]
    fn test_literal_list_of_different_types() {
        assert_eq!(t("[ False [ ] True [ 1 2 ] ]"), "[False,[],True,[1,2]]")
    }

    #[test]
    fn test_literal_string() {
        assert_eq!(
            t("\" [ so { not if ] and } \""),
            "\"[ so { not if ] and }\""
        )
    }

    #[test]
    fn test_literal_block() {
        assert_eq!(t("{ 20 10 + }"), "{ 20 10 + }")
    }

    #[test]
    fn test_literal_list_of_blocks() {
        assert_eq!(
            t("[ { + } { 10 + } { 20 10 + } ]"),
            "[{ + },{ 10 + },{ 20 10 + }]"
        )
    }
}







