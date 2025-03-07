# Performance Tests

## Getting started
First, compile docker image with practrand and binaries from library and run the image.
```bash
docker build -t lichspiel . && docker --rm -it run lichspiel
```
Now there can be several performances tests run.

## Runs

- Test individual RNG if they create random values

    ```bash
    rng | hexdump -Cv | head
    ```

- Compare throughput between algorithms e.g. rng and xorshiftstar

    ```bash
    hyperfine --warmup 3 'rng | head -n 999999' 'xorshiftstar | head -n 999999'
    ```

- Compare statistical stability of the RNG e.g. xorshiftstar

    ```bash
    xorshiftstar | RNG_Test stdin32
    ```