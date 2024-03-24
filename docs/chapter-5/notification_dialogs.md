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

Error dialogs are often use in scenarios where an error has occured and a specific action need to pe stop because of it. There are however cases where you will also want a retry option (if an error occurs, retry the same operation in the hope of another result). If this is the case, the following method can be used:
```rs
fn dialogs::retry(title: &str, caption: &str) -> bool {...}
```

This method will create an error dialog but with two buttons (`Retry` and `Cancel`). If you click on `Retry` button the method will return **true** otherwise it will return **false**. For example, the following code:
```rs
if dialogs::retry("Error","An error occured while performn a copy operation.\nRetry again ?") {
    // retry the operation
}
```
will create a dialog that looks like the following picture:
<img src="img/retry.png" width=300/>

## Alerts

Alerts are dialogs where an error has occured, but it is not that relevant for the program exection flow (its an error from where we can recover).  You can show an error by using the following method:
```rs
fn dialogs::alert(title: &str, caption: &str) {...}
```
This will create a modal window with the message (the content of variable `caption`) provided to this method and one button (that contains the caption `Ok`). The following code:

```rs
dialogs::alert("Error","An error has occured during the last operation");
```

will produce the following error modal window:
<img src="img/alert.png" width=300/>

Just like in the case of errors, if the alert is something we can ignore and continue with the execution, the following method can be used:
```rs
fn dialogs::proceed(title: &str, caption: &str) -> bool {...}
```

This method will create an alert dialog but with two buttons (`Yes` and `No`). If you click on `Yes` button the method will return **true** otherwise it will return **false**. For example, the following code:
```rs
if dialogs::proceed("Alert","An error occured while performn a copy operation.\nContinue anyway ?") {
    // retry the operation
}
```
will create a dialog that looks like the following picture:
<img src="img/proceed.png" width=300/>


## Popup messages

Popup messages are notification of success or generic information that are provided. To show a simple message use the following method:
```rs
fn dialogs::message(title: &str, caption: &str) {...}
```
This will create a modal window with the message (the content of variable `caption`) provided to this method and one button (that contains the caption `Ok`). The following code:

```rs
dialogs::message("Success","All files have been copied");
```

will produce the following modal window:
<img src="img/message.png" width=300/>

## Validation messages

Validation messages are simple questions that determine how the execution flow should continue from that point. To show a validation message use the following method:
```rs
fn dialogs::validate(title: &str, caption: &str) -> bool {...}
```
This method will create a dialog with two buttons (`Yes` and `No`). If you click on `Yes` button the method will return **true** otherwise it will return **false**. This is used to create a simple validation message such `Are you sure you want to proceed ?`.

For example, the following code:
```rs
if dialogs::validate("Question","Are you sure you want to proceed ?") {
    // start the action
}
```
will create a dialog that looks like the following picture:
<img src="img/validate.png" width=300/>

Aditionally a `try_validate` method is also available with the following definition:
```rs
fn dialogs::try_validate(title: &str, caption: &str) -> Option<bool> {...}
```
This method will display three button (`Yes`, `No` and `Cancel`). The result of this dialog are:
* **Some(true)** if `Yes` button is being clicked
* **Some(false)** if `No` button is being clicked
* **None** if `Cancel` button is clicked (or escape key is pressed).

This type of dialog should be used for scenarios where you can do one action in two ways or you can stop doing that action. For example, when an application ends and you need to save the date, you can chose between:
* saving the data (and close the application)
* not saving the date (and still close the application)
or
* cancel (meaning that you will not close the application)

The following code describes a similar scenario:
```rs
if let Some(save_files) = dialogs::try_validate("Exit","Do you want to save your files ?") {
    if save_files {
        // save files and then exist application
    } else {
        // exit the application directly
    }
} else {
    // don't exit the application
}
```
and should create a dialog that looks like the following picture:
<img src="img/try_validate.png" width=300/>