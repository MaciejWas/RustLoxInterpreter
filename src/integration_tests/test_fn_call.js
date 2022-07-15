fun foo() {
    return 0;
}
var x = foo();
print foo();


fun foo(bar) {
    return bar;
}
var x = foo(1);
print foo(1);


fun foo(bar, baz) {
    return bar + baz;
}
var x = foo(1, 2);
print foo(1, 2);


fun foo(bar, baz) {
    bar + baz;
    return bar + baz;
}
var x = foo(1, 2);
print foo(1, 2);


fun foo2(bar, baz) {
    foo(bar, baz);
    return bar + baz;
}
var x = foo2(1, 2);
print foo2(1, 2);


fun foo2(bar, baz) {
    if (bar) {
        return baz;
    }
    return bar;
}
var x = foo2(1, 2);
print foo2(1, 2);