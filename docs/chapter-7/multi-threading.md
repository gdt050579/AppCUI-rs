# Multi-threading

While AppCUI runs on a single thread, multi-threading support is available in some scenarios, such as:
- Timers
- Background tasks

**Remarks:** Multi-threading support relies heavily on channels and on how terminals are implemented on the current operating system. Some features might not work as expected on some terminals, in particular if they are not designed for use with multiple threads.