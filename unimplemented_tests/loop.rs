mod test_loop {
    use stack_interpreter::t;

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