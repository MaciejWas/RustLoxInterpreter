
"Program is a list of statements. Statemant can be an expression.";
3; // like this

// value can be of 3 types:
print true;
print 1;
print "fsdaf";


if (3==3) {
    print "You should see this!";
};

if (1==2) {
    print "You should NOT see this";
};

var x = 0;
while (x != 5) {
    print "Hey, I'm increasing x";
    var x = x + 1;
};
print x;

fun add(x, y) {
    print "hey im inside a function :))";
    return x + y;
};

var y = add(1, 2);
print y;

var x = sfdsa();