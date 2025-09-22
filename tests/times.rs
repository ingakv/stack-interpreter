mod test_times {
    use stack_interpreter::t;

    #[test]
    fn test_times_block() {
        assert_eq!(t("1 times { 100 50 + }"), "150");
    }

    #[test]
    fn test_times_block_with_list() {
        assert_eq!(t("5 times { 1 } [ ] 5 times { append } 0 foldl { + }"), "5");
    }

    #[test]
    fn test_times_condensed_with_list() {
        assert_eq!(t("5 times 1 [ ] 5 times append 0 foldl +"), "5");
    }

    #[test]
    fn test_times_block_addition() {
        assert_eq!(t("5 times { 10 } + + + +"), "50");
    }

    #[test]
    fn test_times_condensed_addition() {
        assert_eq!(t("5 times 10 4 times +"), "50");
    }
}