use heapless::Vec;
use proptest::prelude::*;
use vax_number::*;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 10000,
        .. ProptestConfig::default()
    })]
    #[test]
    fn test_encode_decode(s in "[PM]{0,21}") {
        let s0: Vec<char, 21> = s.chars().collect();
        let x = encode(&s0);
        let s1 = decode(x);
        assert_eq!(s0, s1);
    }
}
