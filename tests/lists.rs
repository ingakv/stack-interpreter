mod test_lists {
    use stack_interpreter::t;

    #[test]
    fn test_list_creation() {
        assert_eq!(t("[ 1 2 3 ]"), "[1,2,3]");
    }

    #[test]
    fn test_mixed_list_creation() {
        assert_eq!(t("[ 1 \" bob \" ]"), "[1,\"bob\"]");
    }

    #[test]
    fn test_list_empty_false() {
        assert_eq!(t("[ 1 2 ] empty"), "False");
    }

    #[test]
    fn test_list_empty_true() {
        assert_eq!(t("[ ] empty"), "True");
    }

    #[test]
    fn test_list_head() {
        assert_eq!(t("[ 1 2 3 ] head"), "1");
    }

    #[test]
    fn test_list_length() {
        assert_eq!(t("[ 1 2 3 ] length"), "3");
    }

    #[test]
    fn test_list_tail() {
        assert_eq!(t("[ 1 2 3 ] tail"), "[2,3]");
    }

    #[test]
    fn test_list_cons() {
        assert_eq!(t("1 [ ] cons"), "[1]");
    }

    #[test]
    fn test_list_cons_append() {
        assert_eq!(t("1 [ 2 3 ] append"), "[1,2,3]");
    }

    #[test]
    fn test_list_append() {
        assert_eq!(t("[ 1 ] [ 2 3 ] cons"), "[1,2,3]");
    }

    #[test]
    fn test_list_append_empty() {
        assert_eq!(t("[ 1 2 ] [ ] cons"), "[1,2]");
    }

    #[test]
    fn test_list_nested_cons() {
        assert_eq!(t("[ 1 ] [ 2 3 ] append"), "[[1],2,3]");
    }
}