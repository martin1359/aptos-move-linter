module NamedAddr::Detector {
    fun func1(x: u128) {

        // let _u64_min = 0;
        // let _u32_max: u32 = 4294967295;
        // let _u32_min = 0;
        // let _u16_max: u16 = 65535;
        // let _u16_min = 0;
        // let _u8_max: u8 = 255;
        // let _u8_min = 0;
        let u128_max: u128 = 340282366920938463463374607431768211455;
        let u128_min = 0;

        if (x > u128_max){

        };

        if (u128_max <  x){

        };
        if (x < u128_min){

        };

        if (u128_min > x){

        };

        if (x < u128_max){

        };
    }
}
