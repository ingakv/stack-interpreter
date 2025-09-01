mod test_list_codeblocks {
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

mod test_if_with_codeblocks {
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

mod test_if_without_codeblock {
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

mod test_loop {
    use bprog::t;

    #[test]
    fn test_loop_with_conditional() {
        assert_eq!(
            t("1 loop { dup 4 > } { dup 1 + } [ ] 5 times { append }"),
            "[1,2,3,4,5]"
        );
    }

    #[test]
    fn test_loop_condensed_with_conditional() {
        assert_eq!(
            t("1 loop { dup 4 > } { dup 1 + } [ ] 5 times append"),
            "[1,2,3,4,5]"
        );
    }

    #[test]
    fn test_loop_with_conditional_length() {
        assert_eq!(
            t("[ 1 ] loop { dup length 9 > } { dup head 1 + swap append }"),
            "[10,9,8,7,6,5,4,3,2,1]"
        );
    }

    #[test]
    fn test_odd_function_false_case() {
        assert_eq!(t("odd { dup 2 div swap 2 / == if False True } fun 2 odd"), "False");
    }

    #[test]
    fn test_odd_function_true_case() {
        assert_eq!(t("odd { dup 2 div swap 2 / == if False True } fun 3 odd"), "True");
    }

    #[test]
    fn test_to_list_function() {
        assert_eq!(t("toList { [ ] swap times append } fun 1 2 3 4 4 toList"), "[1,2,3,4]");
    }

    #[test]
    fn test_gen1to_num_function_sum() {
        assert_eq!(t("gen1toNum { ' max swap := 1 loop { dup max > } { dup 1 + } } fun 3 gen1toNum + + +"), "10");
    }

    #[test]
    fn test_gen1to_num_function_sum_with_gte() {
        assert_eq!(t("gen1toNum { ' max swap := 1 loop { dup max >= } { dup 1 + } } fun 3 gen1toNum + +"), "6");
    }

    #[test]
    fn test_odd_to_list_gen1to_num_functions_combined() {
        assert_eq!(t("odd { dup 2 div swap 2 / == if False True } fun toList { [ ] swap times append } fun gen1toNum { ' max swap := 1 loop { dup max > } { dup 1 + } } fun 4 gen1toNum 5 toList map odd"), "[True,False,True,False,True]");
    }
}

mod test_functions {
    use bprog::t;

    #[test]
    fn test_odd_function() {
        assert_eq!(
            t("odd { dup 2 div swap 2 / == if False True } fun \
                  2 odd"),
            "False"
        );
    }

    #[test]
    fn test_odd_function_true_case() {
        assert_eq!(
            t("odd { dup 2 div swap 2 / == if False True } fun \
                  3 odd"),
            "True"
        );
    }

    #[test]
    fn test_to_list_function() {
        assert_eq!(
            t("toList { [ ] swap times append } fun \
                  1 2 3 4 \
                  4 toList"),
            "[1,2,3,4]"
        );
    }

    #[test]
    fn test_gen1to_num_function() {
        assert_eq!(
            t(
                "gen1toNum { max swap := 1 loop { dup max > } { dup 1 + } } fun \
                  3 gen1toNum + + +"
            ),
            "10"
        );
    }

    #[test]
    fn test_odd_to_list_gen1to_num_functions_combined() {
        assert_eq!(
            t("odd { dup 2 div swap 2 / == if False True } fun \
                  toList { [ ] swap times append } fun \
                  gen1toNum { max swap := 1 loop { dup max > } { dup 1 + } } fun \
                  4 gen1toNum 5 toList map odd"),
            "[True,False,True,False,True]"
        );
    }

    #[test]
    fn test_inc_function() {
        assert_eq!(t("inc { 1 + } fun 1 inc"), "2");
    }

    #[test]
    fn test_mul10_and_inc_functions() {
        assert_eq!(t("mul10 { 10 * } fun inc { 1 + } fun 10 inc mul10"), "110");
    }
    
    #[test]
    fn test_drop_function() {
        assert_eq!(t("drop { times tail } fun [ 1 2 3 4 5 ] 3 drop"), "[4,5]");
    }

}
