mod test_string_parsing {
    use bprog::t;

    #[test]
    fn test_parse_integer() {
        assert_eq!(t("\" 12 \" parseInteger"), "12");
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(t("\" 12.34 \" parseFloat"), "12.34");
    }

    #[test]
    fn test_words() {
        assert_eq!(
            t("\" adam bob charlie \" words"),
            "[\"adam\",\"bob\",\"charlie\"]"
        );
    }
}