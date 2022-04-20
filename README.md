# text-log-parser
This is a simple text log parser that works for any delimited log file.
The default delimiter is space ` ` but can be configured by changing the LogFormat delimiter attribute when you create your format.

Check out the test: https://github.com/vinodotdev/text-log-parser/blob/main/tests/test_parse.rs

To name the fields in the log, you should use `$variable_name` syntax.  Again, see the example.  The parser will use the format provided to create a structure of seperators and automatically attempt to determine the delimiters.
