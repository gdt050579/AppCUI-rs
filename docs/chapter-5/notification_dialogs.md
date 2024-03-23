# Notification Dialogs

Notification dialogs are predefined modal window that can be used for various purposes such as:
* show an error or a warning
* provide a validation (where you need to acknoledge a certain action)
* show a message
* etc

## Errors

You can show an error by using the following method:
```rs
fn dialogs::error(title: &str, caption: &str) {...}
```
This will create a modal window with the message provided to this method and one button (that contains the caption `Ok`). The following code:

```rs
dialogs::error("Error","An error has occured during the last operation");
```

will produce the following error modal window:
<img src="img/error.png" width=300/>