mod test_assignments {
    use stack_interpreter::t;

    #[test]
    fn test_variable_name() {
        assert_eq!(t("age"), "age");
    }

    #[test]
    fn test_variable_assignment() {
        assert_eq!(t("age 10 := age"), "10");
    }

    #[test]
    fn test_variable_assignment_swap() {
        assert_eq!(t("10 age swap := age"), "10");
    }

    #[test]
    fn test_variable_assignment_list() {
        assert_eq!(t("[ 1 2 3 ] list swap := list"), "[1,2,3]");
    }

    #[test]
    fn test_variable_update() {
        assert_eq!(t("age 20 := [ 10 age ]"), "[10,20]");
    }

    #[test]
    fn test_assignments_quote() {
        assert_eq!(t("' age"), "age");
    }

    #[test]
    fn test_assignments_reassign() {
        assert_eq!(t("age 10 := ' age 20 := age"), "20");
    }

    #[test]
    fn test_assignments_eval() {
        assert_eq!(t("age 10 := ' age eval"), "10");
    }

    #[test]
    fn test_assignments_fun_inc() {
        assert_eq!(t("inc { 1 + } fun 1 inc"), "2");
    }

    #[test]
    fn test_assignments_fun_mul10_inc() {
        assert_eq!(t("mul10 { 10 * } fun inc { 1 + } fun 10 inc mul10"), "110");
    }
}