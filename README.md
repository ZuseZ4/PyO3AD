Usage instructions:

```
rustup default enzyme
maturin develop
python3
```
Then test it using
```
import math
import rustnn
math.cos(1.0)
rustnn.cos(1.0)
```
They should be almost identical.
Have a look at the rustnn module, we never call `cos`, only `sin`.
So the cosine comes from differentiating our `sin` call.

