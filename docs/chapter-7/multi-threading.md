# Multi Threading

While AppCUI works on a single thread, the multi-threading support is available in some scenarios such as:
- Timers
- Background tasks

**Remarks**: Multi-threading support relies heavely on channels and the way terminals are implemented on the current operating system. This means that some features might not work as expected on some terminals (in particular if they are not adapted to work with multiple threads).