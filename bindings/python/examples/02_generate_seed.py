import os
import hashlib

rnd_seed = hashlib.sha256(os.urandom(256)).hexdigest()
print(rnd_seed)