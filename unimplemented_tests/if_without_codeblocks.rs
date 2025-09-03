mod test_if_without_codeblock {
    use bprog::t;

    #[test]
    fn test_if_true_condensed() {
        assert_eq!(t("True if 20 { }"), "20");
    }

    #[test]
    fn test_if_true_block_condensed() {
        assert_eq!(t("True if { 20 10 + } 3"), "30");
    }

    #[test]
    fn test_if_condition_condensed() {
        assert_eq!(t("10 10 5 5 == if + { 100 + }"), "20");
    }

    #[test]
    fn test_if_false_condensed() {
        assert_eq!(t("False if { } 45"), "45");
    }

    #[test]
    fn test_if_nested_condensed() {
        assert_eq!(t("True if { False if 50 100 } 30"), "100");
    }
}