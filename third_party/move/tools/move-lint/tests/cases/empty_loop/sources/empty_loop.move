module NamedAddr::Detector {
    use std::vector;

    public fun deep_nesting_check(addr: address, reward_token: u64) {
        let i = 0;
        loop {
            i = i + 1;
        };

        loop {};

        let sum = 0;
        let i = 1;
        let n = 10;
        while (i <= 10) {
            sum = sum + i;
            i = i + 1
        };
        while (i < 5) {
            
        };
        let x = 0;
        while ({ x = x + 1; x < 10}) {};
    }
}