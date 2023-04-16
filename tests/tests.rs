mod test_literals {
    use bprog::t;

    #[test]
    fn test_literal_int() {
        assert_eq!(t("5"), "5")
    }

    #[test]
    fn test_literal_long_int() {
        assert_eq!(t("121231324135634563456363567"), "121231324135634563456363567")
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
        assert_eq!(t("\" [ so { not if ] and } \""), "\"[ so { not if ] and }\"")
    }

    #[test]
    fn test_literal_block() {
        assert_eq!(t("{ 20 10 + }"), "{ 20 10 + }")
    }

    #[test]
    fn test_literal_list_of_blocks() {
        assert_eq!(t("[ { + } { 10 + } { 20 10 + } ]"), "[{ + },{ 10 + },{ 20 10 + }]")
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
        assert_eq!(t("20 2 /"), "10.0");
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
        assert_eq!(t("20 2.0 div"), "10");
    }

    #[test]
    fn test_float_division_with_float() {
        assert_eq!(t("20.0 2.0 div"), "10");
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
        assert_eq!(t("\" adam bob charlie \" words"), "[\"adam\",\"bob\",\"charlie\"]");
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
        assert_eq!(t("1 [ 2 3 ] cons"), "[1,2,3]");
    }

    #[test]
    fn test_list_append() {
        assert_eq!(t("[ 1 ] [ 2 3 ] append"), "[1,2,3]");
    }

    #[test]
    fn test_list_append_empty() {
        assert_eq!(t("[ 1 2 ] [ ] append"), "[1,2]");
    }

    #[test]
    fn test_list_nested_cons() {
        assert_eq!(t("[ 1 ] [ 2 3 ] cons"), "[[1],2,3]");
    }
}

mod test_list_quotations {
    use bprog::t;

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
        assert_eq!(t("[ 1 2 3 4 ] map { dup 2 > if { 10 * } { 2 * } }"), "[2,4,30,40]");
    }

    #[test]
    fn test_each_add() {
        assert_eq!(t("[ 1 2 3 4 ] each { 10 * } + + +"), "100");
    }

    #[test]
    fn test_foldl_sum() {
        assert_eq!(t("[ 1 2 3 4 ] 0 foldl { + }"), "10");
    }

    #[test]
    fn test_foldl_div() {
        assert_eq!(t("[ 2 5 ] 20 foldl { div }"), "2");
    }

    #[test]
    fn test_each_parse_integer() {
        assert_eq!(t("[ \" 1 \" \" 2 \" \" 3 \" ] each { parseInteger } [ ] cons cons cons"), "[1,2,3]");
    }

    #[test]
    fn test_each_parse_integer_3_times() {
        assert_eq!(t("[ \" 1 \" \" 2 \" \" 3 \" ] each parseInteger [ ] 3 times cons"), "[1,2,3]");
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

mod test_assignments {
    use bprog::t;
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

}

mod test_quotations {
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
    fn test_exec_nested_block() {
        assert_eq!(t("{ { 10 20 + } exec } exec"), "30");
    }

    #[test]
    fn test_exec_nested_block_with_add() {
        assert_eq!(t("{ { 10 20 + } exec 20 + } exec"), "50");
    }
}

mod test_if {
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

mod test_if_without_quotation {
    use bprog::t;

    #[test]
    fn test_if_true_condensed() {
        assert_eq!(t("True if 20 { }"), "20");
    }

    #[test]
    fn test_if_true_block_condensed() {
        assert_eq!(t("True if { 20 10 + } 3"), "30");
    }

    #[test]
    fn test_if_condition_condensed() {
        assert_eq!(t("10 10 5 5 == if + { 100 + }"), "20");
    }

    #[test]
    fn test_if_false_condensed() {
        assert_eq!(t("False if { } 45"), "45");
    }

    #[test]
    fn test_if_nested_condensed() {
        assert_eq!(t("True if { False if 50 100 } 30"), "100");
    }
}

mod test_times {
    use bprog::t;

    #[test]
    fn test_times_block() {
        assert_eq!(t("1 times { 100 50 + }"), "150");
    }

    #[test]
    fn test_times_block_with_list() {
        assert_eq!(t("5 times { 1 } [ ] 5 times { cons } 0 foldl { + }"), "5");
    }

    #[test]
    fn test_times_condensed_with_list() {
        assert_eq!(t("5 times 1 [ ] 5 times cons 0 foldl +"), "5");
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

mod test_loop {
    use bprog::t;

    #[test]
    fn test_loop_with_conditional() {
        assert_eq!(t("1 loop { dup 4 > } { dup 1 + } [ ] 5 times { cons }"), "[1,2,3,4,5]");
    }

    #[test]
    fn test_loop_condensed_with_conditional() {
        assert_eq!(t("1 loop { dup 4 > } { dup 1 + } [ ] 5 times cons"), "[1,2,3,4,5]");
    }

    #[test]
    fn test_loop_with_conditional_length() {
        assert_eq!(t("[ 1 ] loop { dup length 9 > } { dup head 1 + swap cons }"), "[10,9,8,7,6,5,4,3,2,1]");
    }
}

mod test_functions {
    use bprog::t;

    #[test]
    fn test_odd_function() {
        assert_eq!(t("odd { dup 2 div swap 2 / == if False True } fun \
                  2 odd"), "False");
    }

    #[test]
    fn test_odd_function_true_case() {
        assert_eq!(t("odd { dup 2 div swap 2 / == if False True } fun \
                  3 odd"), "True");
    }

    #[test]
    fn test_to_list_function() {
        assert_eq!(t("toList { [ ] swap times cons } fun \
                  1 2 3 4 \
                  4 toList"), "[1,2,3,4]");
    }

    #[test]
    fn test_gen1to_num_function() {
        assert_eq!(t("gen1toNum { max swap := 1 loop { dup max > } { dup 1 + } } fun \
                  3 gen1toNum + + +"), "10");
    }

    #[test]
    fn test_odd_to_list_gen1to_num_functions_combined() {
        assert_eq!(t("odd { dup 2 div swap 2 / == if False True } fun \
                  toList { [ ] swap times cons } fun \
                  gen1toNum { max swap := 1 loop { dup max > } { dup 1 + } } fun \
                  4 gen1toNum 5 toList map odd"), "[True,False,True,False,True]");
    }

    #[test]
    fn test_inc_function() {
        assert_eq!(t("inc { 1 + } fun 1 inc"), "2");
    }

    #[test]
    fn test_mul10_and_inc_functions() {
        assert_eq!(t("mul10 { 10 * } fun inc { 1 + } fun 10 inc mul10"), "110");
    }
}