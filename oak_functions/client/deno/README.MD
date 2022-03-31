Start server via

```
xtask --scope=all --logs run-oak-functions-examples --example-name=weather_lookup --client-variant=none
```

Then run client via

```
deno run --allow-net oak_functions/client/deno/hello.ts
```
