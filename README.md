# DelSquared-Script
DelSquared-Script. A very basic interpreted scripting language written in Rust. This project is recreational.

# Goals For This Language
Mainly I am leaning towards keeping it as minimalistic as possible (syntactically speaking). In the future once everthing is a bit settled in I might use it to natively be very fast at numerical computation but that's just wishfull thinking for now.

# Installation
So far, just clone and `cargo build`. As can be seen from the `.toml` there are no dependencies so far. This may change in the future but I will try to use as few dependencies as possible.

# Documentation
So far the language literally does nothing except from reading the script file and prints some debug output. Will update this space once there's something worthy of mentioning.

# Current Debug Output with example `script.ds`

In case anyone's interested to compare

```

===== In file: =====

script.ds


===== With text: =====

a = 5
b = 10

print(a + b)


Elapsed: 73.00µs


===== Split text: =====

["a = 5\r", "b = 10\r", "print(a + b)\r"]

Correct use of brackets: true

Elapsed: 26.70µs


===== Var table tests: =====


{}

{
    "test_var": 100,
}

{
    "test_var2": 10.1,
    "test_var": 100,
}

{
    "test_var2": 10.1,
}

{}

Elapsed: 306.00µs


===== Var table tests: =====


{}

{"test_var": true}

{"test_var": true, "test_var2": false}

{"test_var2": false}

{}

Elapsed: 41.00µs


===== Keywords: =====

["(\\w*)\\((.*)\\)"]

Elapsed: 900.00ns


===== Keywords: =====

["if", "else", "end", "label", "goto"]

Elapsed: 1.80µs


===== Binary Numerical Operators: =====

["-", "*", "+", "/"]

Elapsed: 13.80µs


===== Logical Operators: =====

["^", "|", "!", "&"]

Elapsed: 11.30µs


===== Binary Comparative Operators: =====

["<=", ">", ">=", "==", "!=", "<"]

Elapsed: 12.70µs


===== Control Operators: =====

["~", "="]

Elapsed: 6.40µs


```
