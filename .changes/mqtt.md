
---
"nodejs-binding": patch
---

Fix MQTT multiple events when .listen() is called multiple times.
Made `Client::listen()` async.
Added `Client::clearListeners()`.