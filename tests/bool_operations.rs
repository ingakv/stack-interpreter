mod test_bool_operations {
    use bprog::t;

    #[test]
    fn test_and_operation() {
        assert_eq!(t("False False &&"), "False");
    }

    #[test]
    fn test_or_operation() {
        assert_eq!(t("False True ||"), "True");
    }

    #[test]
    fn test_not_operation_false() {
        assert_eq!(t("False not"), "True");
    }

    #[test]
    fn test_not_operation_true() {
        assert_eq!(t("True not"), "False");
    }
}