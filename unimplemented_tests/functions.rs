mod test_functions {
    use stack_interpreter::t;

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
