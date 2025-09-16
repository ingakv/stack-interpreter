mod test_stack_operations {
    use stack_interpreter::t;

    #[test]
    fn test_swap_pop() {
        assert_eq!(t("10 20 swap pop"), "20");
    }

    #[test]
    fn test_dup_swap_pop() {
        assert_eq!(t("10 dup dup + swap pop"), "20");
    }

    #[test]
    fn test_swap_dup_div() {
        assert_eq!(t("10 20 swap dup + div"), "1");
    }
}