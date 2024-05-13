module NamedAddr::counter {

    struct Counter has key { i: u64, z: u64 }


    public fun scope_nested_test(addr: address) acquires Counter {
        let x = 3;
        let y = 3;
        let z = 3;
        let m = 3;
        if( x > 4){

        };
        let c_ref = borrow_global_mut<Counter>(addr);
        c_ref.i = 3;
        let d_ref = borrow_global_mut<Counter>(addr);
        function_take_mut(&mut d_ref.i)
    }

    public fun function_take_mut(i: &mut u64) {
        
    }

    public inline fun scope_nested_test1(addr: address) acquires Counter {
        let c_ref = borrow_global_mut<Counter>(addr);
        c_ref.i = 3;
        let _d_ref = borrow_global_mut<Counter>(addr);


    }


}