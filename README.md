# DelSquared-Script
DelSquared-Script. A very basic interpreted scripting language written in Rust. This project is recreational.

# Goals For This Language
Mainly I am leaning towards keeping it as minimalistic as possible (syntactically speaking). In the future once everthing is a bit settled in I might use it to natively be very fast at numerical computation but that's just wishfull thinking for now.

# Installation
So far, just clone and `cargo build`. As can be seen from the `.toml` there are no dependencies so far. This may change in the future but I will try to use as few dependencies as possible.

# Documentation
So far the language literally does nothing except from reading the script file and prints some debug output. Will update this space once there's something worthy of mentioning.

# Current Debug Output with examples

In case anyone's interested to compare

script.ds (not working anymore but will be available again soon, this output is with past versions)

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

rpn_test.ds

```

===== In file: =====

tests/rpn_test.ds


===== With text: =====

a : num = 5
b : num = 6

RPN : 2 2 + 1 - = first_eval

RPN : 2 2 2 2 $ 5 5 £ 10 - = second_eval

RPN : a b + a * b * = third_eval


Elapsed: 96.60µs


===== Split text: =====

["a : num = 5", "b : num = 6", "RPN : 2 2 + 1 - = first_eval", "RPN : 2 2 2 2 $ 5 5 £ 10 - = second_eval", "RPN : a b + a * b * = third_eval"]

Correct use of brackets: true

Elapsed: 21.40µs


===== Var table tests: =====


{}

{"test_var": 100.0}

{"test_var2": 10.1, "test_var": 100.0}

{"test_var2": 10.1}

{}

Elapsed: 377.20µs


===== Var table tests: =====


{}

{"test_var": true}

{"test_var2": false, "test_var": true}

{"test_var2": false}

{}

Elapsed: 69.30µs


===== Keywords: =====

["(\\w*)\\((.*)\\)"]

Elapsed: 2.20µs


===== Keywords: =====

["if", "else", "end", "label", "goto", "RPN", "dumpstack"]

Elapsed: 2.20µs


===== Binary Numerical Operators: =====

["+", "-", "*", "%", "/"]

Elapsed: 11.10µs


===== Stack Numerical Operators: =====

["£", "$"]

Elapsed: 9.90µs


===== Logical Operators: =====

["^", "&", "!", "|"]

Elapsed: 7.00µs


===== Binary Comparative Operators: =====

["==", "!=", "<=", ">=", "<", ">"]

Elapsed: 8.20µs


===== Control Operators: =====

["=", "~"]

Elapsed: 3.40µs


===== SCRIPT EXECUTION: =====


Line : "a : num = 5"

Numtable : {"a": 5.0}

Line : "b : num = 6"

Numtable : {"a": 5.0, "b": 6.0}

Line : "RPN : 2 2 + 1 - = first_eval"

"2 2 + 1 - = first_eval"

parse : "first_eval"
"2 2 + 1 -"

tkn = "2"
[2.0]
tkn = "2"
[2.0, 2.0]
tkn = "+"
[4.0]
tkn = "1"
[4.0, 1.0]
tkn = "-"
[3.0]
result = [3.0] (quick maths)
Numtable : {"a": 5.0, "first_eval": 3.0, "b": 6.0}

Line : "RPN : 2 2 2 2 $ 5 5 £ 10 - = second_eval"

"2 2 2 2 $ 5 5 £ 10 - = second_eval"

parse : "second_eval"
"2 2 2 2 $ 5 5 £ 10 -"

tkn = "2"
[2.0]
tkn = "2"
[2.0, 2.0]
tkn = "2"
[2.0, 2.0, 2.0]
tkn = "2"
[2.0, 2.0, 2.0, 2.0]
tkn = "$"
[8.0]
tkn = "5"
[8.0, 5.0]
tkn = "5"
[8.0, 5.0, 5.0]
tkn = "£"
[200.0]
tkn = "10"
[200.0, 10.0]
tkn = "-"
[190.0]
result = [190.0] (quick maths)
Numtable : {"first_eval": 3.0, "b": 6.0, "a": 5.0, "second_eval": 190.0}

Line : "RPN : a b + a * b * = third_eval"

"a b + a * b * = third_eval"

parse : "third_eval"
"a b + a * b *"

tkn = "a"
[5.0]
tkn = "b"
[5.0, 6.0]
tkn = "+"
[11.0]
tkn = "a"
[11.0, 5.0]
tkn = "*"
[55.0]
tkn = "b"
[55.0, 6.0]
tkn = "*"
[330.0]
result = [330.0] (quick maths)
Numtable : {"first_eval": 3.0, "b": 6.0, "a": 5.0, "second_eval": 190.0, "third_eval": 330.0}


===== SCRIPT EXECUTION COMPLETE =====




```
