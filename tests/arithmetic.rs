mod test_simple_arithmetic {
    use bprog::t;

    #[test]
    fn test_addition() {
        assert_eq!(t("1 1 +"), "2");
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(t("10 20 *"), "200");
    }

    #[test]
    fn test_division() {
        assert_eq!(t("20 2 div"), "10");
    }

    #[test]
    fn test_float_division() {
        assert_eq!(t("20.0 2 /"), "10.0");
    }
}

mod test_arithmetic_with_type_coercion {
    use bprog::t;

    #[test]
    fn test_addition_with_float() {
        assert_eq!(t("1 1.0 +"), "2.0");
    }

    #[test]
    fn test_multiplication_with_float() {
        assert_eq!(t("10 20.0 *"), "200.0");
    }

    #[test]
    fn test_division_with_float() {
        assert_eq!(t("20.0 2 div"), "10.0");
    }

    #[test]
    fn test_bool_int_equality() {
        assert_eq!(t("True 0 + False 0 + =="), "False");
    }
}