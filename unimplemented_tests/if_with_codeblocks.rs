mod test_if_with_codeblocks {
    use bprog::t;

    #[test]
    fn test_if_true() {
        assert_eq!(t("True if { 20 } { }"), "20");
    }

    #[test]
    fn test_if_true_block() {
        assert_eq!(t("True if { 20 10 + } { 3 }"), "30");
    }

    #[test]
    fn test_if_condition() {
        assert_eq!(t("10 5 5 == if { 10 + } { 100 + }"), "20");
    }

    #[test]
    fn test_if_false() {
        assert_eq!(t("False if { } { 45 }"), "45");
    }

    #[test]
    fn test_if_nested() {
        assert_eq!(t("True if { False if { 50 } { 100 } } { 30 }"), "100");
    }
}