module NamedAddr::Detector {
    use aptos_framework::randomness;
    // public fun func1() {
    //     let _winner_idx = randomness::u64_range(0, 5);
    // }

    public entry fun func2() {
        let _winner_idx = randomness::u64_range(0, 5);
    }
}
