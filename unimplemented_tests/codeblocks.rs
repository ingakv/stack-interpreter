mod test_codeblocks {
    use bprog::t;

    #[test]
    fn test_exec_block() {
        assert_eq!(t("{ 20 10 + } exec"), "30");
    }

    #[test]
    fn test_exec_block_with_value() {
        assert_eq!(t("10 { 20 + } exec"), "30");
    }

    #[test]
    fn test_exec_block_with_two_values() {
        assert_eq!(t("10 20 { + } exec"), "30");
    }

    #[test]
    fn test_nested_codeblock_exec() {
        assert_eq!(t("{ { print } exec }"), "{ { print } exec }");
    }

    #[test]
    fn test_exec_nested_block() {
        assert_eq!(t("{ { 10 20 + } exec } exec"), "30");
    }

    #[test]
    fn test_exec_nested_block_with_add() {
        assert_eq!(t("{ { 10 20 + } exec 20 + } exec"), "50");
    }
}