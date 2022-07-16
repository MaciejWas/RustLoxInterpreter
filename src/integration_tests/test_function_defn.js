fun foo() {
    return 0;
};

fun foo(bar) {
    return bar;
};

fun foo(bar, baz) {
    return bar + baz;
};

fun foo(bar, baz) {
    bar + baz;
    return bar + baz;
};

fun foo2(bar, baz) {
    foo(bar, baz);
    return bar + baz;
};

fun foo2(bar, baz) {
    if (bar) {
        return baz;
    };
    return bar;
};

class xd {
    var x = "d";

    fun xddd() {
        return 0;
    };
};