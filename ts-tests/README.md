# Functional testing for Substrate metaverse Node RPC

This folder contains a set of functional tests desgined to perform functional testing on the metaverse Eth RPC.

It is written in typescript, using Mocha/Chai as Test framework.

## Test flow

Tests are separated depending of their genesis requirements.
Each group will start a [metaverse test node](metaverse-test-node) with a given [spec](substrate-specs) before executing the tests.

## Installation

```
npm install
```

## Run the tests

```
npm run test
```

You can also add the metaverse Node logs to the output using the `METAVERSE_LOG` env variable. Ex:

```
METAVERSE_LOG="warn,rpc=trace" npm run test
```

(The metaverse node be listening for RPC on port 19933, mostly to avoid conflict with already running substrate node)
