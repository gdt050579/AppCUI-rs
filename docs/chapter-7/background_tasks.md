# Background Tasks

A background task is a thread that can comunicate with the main AppCUI threat and send/receive information from it, just like in the following diagram:

<img src="img/background_task.png" />

To start a background task, use the following command:

```rs
pub struct BackgroundTask<T,T>
where: 
    T: Send + 'static, 
    R: Send + 'static 
{ ... }

impl<T: Send, R: Send> BackgroundTask<T, R> {
    pub fn run(task: fn(conector: &BackgroundTaskConector<T, R>), 
               receiver: Handle<Window>) -> Handle<BackgroundTask<T, R>> 
    {...}
}
```

where:
* `T` represents the type that will be send from the background thread to the main thread (the thread where AppCUI runs). It is usually an enum that reflects a status (e.g. precentage of the task) or a query (information needed by the background thread)
* `R` represent the response for a query (meaning that if the background thread ask the main thread using type **T**, the reply will be of type **R**)