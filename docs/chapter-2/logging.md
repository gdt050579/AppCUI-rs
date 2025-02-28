# Logging

AppCUI supports an internal logging mechanism that can be used to log messages to a file. The logging mechanism is available only in **debug mode** and can be used by calling the `log!` macro. The macro has the following syntax:

```rs
log!(TAG, format, ...)
```

To enable the logging mechanism, you need to specify a log file when creating the application. This can be done by calling the `log_file` method when AppCUI is initialized This method has two parameters: the path to the log file and a boolean value that specifies if the log file should be appended or overwritten. 

```rs
AppCUI::new().log_file("debug.log", false)
             .build()
             .expect("Fail to create an AppCUI application");
```

## Example

```rs
let x = 10;
log!("INFO", "The value of x is: {}", x);
```

Logging mechanism has zero overhead when the application is compiled in **release mode**. The logging mechanism is disabled in **release mode** and the `log!` macro will not generate any code.