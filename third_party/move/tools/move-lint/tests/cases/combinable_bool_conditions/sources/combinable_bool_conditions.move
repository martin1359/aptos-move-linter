module NamedAddr::Detector {
    const ERROR_NUM: u64 = 2;
    public fun func1(x: u64, y: u64, z: u64) {
        let m = 3;
        let n = 4;
        if (x == y || z < y) {};
        if (x < y || x == y) {}; // should be x <= y
        if (x == y || x > y) {}; // should be x >= y
        if (x > y || x == y) {}; // should be x >= y
        if (x != y || x < y) {}; // same as x < y
        if (x < y || x != y) {}; // same as x < y
        if (x != y || x > y) {}; // same as x > y
        if (x > y || x != y) {}; // same as x > y
        if (x > y || y != x) {}; // same as x > y

        if (m == n || m < n) {}; // should be m <= n

        if (x <= y) {};
        if (x >= y) {};
        if (x > y) {};
        if (x < y) {};
        if (x == 11 || x < 3) {};
        if (x == 11 || x < 11) {};
    }
}