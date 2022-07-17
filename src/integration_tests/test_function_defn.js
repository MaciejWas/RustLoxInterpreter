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

fun fib(n) { if (n == 0) { return n; }; if (n==1) { return n; }; return fib(n-1) + fib(n-2); };

print fib(20);