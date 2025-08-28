mod test_literals {
    use bprog::t;

    #[test]
    fn test_literal_int() {
        assert_eq!(t("5"), "5")
    }

    #[test]
    fn test_literal_long_int() {
        assert_eq!(
            t("121231324135634563456363567"),
            "121231324135634563456363567"
        )
    }

    #[test]
    fn test_literal_float() {
        assert_eq!(t("1.0"), "1.0")
    }

    #[test]
    fn test_literal_float_zero() {
        assert_eq!(t("0.0"), "0.0")
    }

    #[test]
    fn test_literal_negative_int() {
        assert_eq!(t("-1"), "-1")
    }

    #[test]
    fn test_literal_negative_float() {
        assert_eq!(t("-1.1"), "-1.1")
    }

    #[test]
    fn test_literal_bool_false() {
        assert_eq!(t("False"), "False")
    }

    #[test]
    fn test_literal_bool_true() {
        assert_eq!(t("True"), "True")
    }

    #[test]
    fn test_literal_nested_list() {
        assert_eq!(t("[ [ ] [ ] ]"), "[[],[]]")
    }

    #[test]
    fn test_literal_list_of_different_types() {
        assert_eq!(t("[ False [ ] True [ 1 2 ] ]"), "[False,[],True,[1,2]]")
    }

    #[test]
    fn test_literal_string() {
        assert_eq!(
            t("\" [ so { not if ] and } \""),
            "\"[ so { not if ] and }\""
        )
    }

    #[test]
    fn test_literal_block() {
        assert_eq!(t("{ 20 10 + }"), "{ 20 10 + }")
    }

    #[test]
    fn test_literal_list_of_blocks() {
        assert_eq!(
            t("[ { + } { 10 + } { 20 10 + } ]"),
            "[{ + },{ 10 + },{ 20 10 + }]"
        )
    }
}

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

mod test_comparison {
    use bprog::t;

    #[test]
    fn test_less_than_operation() {
        assert_eq!(t("20 10 <"), "False");
    }

    #[test]
    fn test_greater_than_operation() {
        assert_eq!(t("20 10 >"), "True");
    }

    #[test]
    fn test_greater_than_operation_with_float() {
        assert_eq!(t("20 10.0 >"), "True");
    }

    #[test]
    fn test_float_greater_than_operation() {
        assert_eq!(t("20.0 20.0 >"), "False");
    }

    #[test]
    fn test_greater_than_or_equal_true() {
        assert_eq!(t("20 10 >="), "True");
    }

    #[test]
    fn test_greater_than_or_equal_false() {
        assert_eq!(t("10 20 >="), "False");
    }

    #[test]
    fn test_greater_than_or_equal_equal_values() {
        assert_eq!(t("10 10 >="), "True");
    }

    #[test]
    fn test_greater_than_or_equal_with_float() {
        assert_eq!(t("20 10.0 >="), "True");
    }

    #[test]
    fn test_greater_than_or_equal_int_float_equal() {
        use bprog::t;
        assert_eq!(t("10 10.0 >="), "True");
    }

    #[test]
    fn test_equality_operation() {
        assert_eq!(t("10 10 =="), "True");
    }

    #[test]
    fn test_equality_operation_with_float() {
        assert_eq!(t("10 10.0 =="), "True");
    }

    #[test]
    fn test_boolean_equality_operation() {
        assert_eq!(t("True True =="), "True");
    }

    #[test]
    fn test_nested_equality_operation() {
        assert_eq!(t("True 40 40 == =="), "True");
    }

    #[test]
    fn test_string_equality_operation() {
        assert_eq!(t("\" abba \" \" abba \" =="), "True");
    }

    #[test]
    fn test_empty_list_equality_operation() {
        assert_eq!(t("[ ] [ ] =="), "True");
    }

    #[test]
    fn test_list_equality_operation() {
        assert_eq!(t("[ 1 2 ] [ 1 2 ] =="), "True");
    }

    #[test]
    fn test_nested_list_equality_operation() {
        assert_eq!(t(" [ [ ] ] [ [ ] ] =="), "True");
    }
}

mod test_stack_operations {
    use bprog::t;

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

mod test_length {
    use bprog::t;

    #[test]
    fn test_hello_length() {
        assert_eq!(t("\" hello \" length"), "5");
    }

    #[test]
    fn test_hello_world_length() {
        assert_eq!(t("\" hello world \" length"), "11");
    }

    #[test]
    fn test_list_length() {
        assert_eq!(t("[ 1 2 3 [ ] ] length"), "4");
    }

    #[test]
    fn test_block_length() {
        assert_eq!(t("{ 10 20 + } length"), "3");
    }
}

mod test_string_parsing {
    use bprog::t;

    #[test]
    fn test_parse_integer() {
        assert_eq!(t("\" 12 \" parseInteger"), "12");
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(t("\" 12.34 \" parseFloat"), "12.34");
    }

    #[test]
    fn test_words() {
        assert_eq!(
            t("\" adam bob charlie \" words"),
            "[\"adam\",\"bob\",\"charlie\"]"
        );
    }
}

mod test_lists {
    use bprog::t;

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