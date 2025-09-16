mod test_list_codeblocks {
    use stack_interpreter::t;

    #[test]
    fn test_map_multiply() {
        assert_eq!(t("[ 1 2 3 ] map { 10 * }"), "[10,20,30]");
    }

    #[test]
    fn test_map_add() {
        assert_eq!(t("[ 1 2 3 ] map { 1 + }"), "[2,3,4]");
    }

    #[test]
    fn test_map_conditional() {
        assert_eq!(
            t("[ 1 2 3 4 ] map { dup 2 > if { 10 * } { 2 * } }"),
            "[2,4,30,40]"
        );
    }

    #[test]
    fn test_each_multiply_append() {
        assert_eq!(t("[ 1 2 3 ] each { 10 * } [ ] append append append"), "[10,20,30]");
    }

    #[test]
    fn test_each_add() {
        assert_eq!(t("[ 1 2 3 4 ] each { 10 * } + + +"), "100");
    }

    #[test]
    fn test_each_add_with_block() {
        assert_eq!(t("10 [ 1 2 3 ] each { + }"), "16");
    }

    #[test]
    fn test_each_add_condensed() {
        assert_eq!(t("10 [ 1 2 3 ] each + "), "16");
    }

    #[test]
    fn test_foldl_sum() {
        assert_eq!(t("[ 1 2 3 4 ] 0 foldl { + }"), "10");
    }

    #[test]
    fn test_foldl_sum_short() {
        assert_eq!(t("[ 1 2 3 4 ] 0 foldl +"), "10");
    }

    #[test]
    fn test_foldl_div() {
        assert_eq!(t("[ 2 5 ] 20 foldl { div }"), "2");
    }

    #[test]
    fn test_each_parse_integer() {
        assert_eq!(
            t("[ \" 1 \" \" 2 \" \" 3 \" ] each { parseInteger } [ ] append append append"),
            "[1,2,3]"
        );
    }

    #[test]
    fn test_each_parse_integer_3_times() {
        assert_eq!(
            t("[ \" 1 \" \" 2 \" \" 3 \" ] each parseInteger [ ] 3 times append"),
            "[1,2,3]"
        );
    }

    #[test]
    fn test_foldl_add_short() {
        assert_eq!(t("[ 1 2 3 4 ] 0 foldl +"), "10");
    }

    #[test]
    fn test_foldl_div_short() {
        assert_eq!(t("[ 2 5 ] 20 foldl div"), "2");
    }
}