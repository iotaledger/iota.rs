---
"nodejs-binding": major
---

Changed input() to accept the output id as string instead of the transaction id and the output index
Add functionality for offline signing: offlineMode(), findInputs(), prepareTransaction(), signTransaction(), finishMessage()