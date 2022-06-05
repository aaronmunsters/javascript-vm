<div align="center">

  <h1><code>javascript-vm</code></h1>

  <strong>A JavaScript interpreter VM that can run in JavaScript, on WebAssembly.</strong>
</div>

## 🚴 Usage

The library exports a single JavaScript function `evaluate` that evaluates source code as a string and returns the evaluation of the last statement or expression. Supported values it can return are:
* `null`
* `undefined`
* `Boolean`
* `string`
* `number`
* `BigInt`
* `Object` (JSON-serialized)
* `Symbol`

Example usage:

```JavaScript
const vmjs = require("javascript-vm");

vmjs.evaluate('"hello" + " " + "world!"'); // === "hello world!"
vmjs.evaluate('({ a: 1, b: 2 })'); // === { a: 1, b: 2 }
vmjs.evaluate(`
function factorial(n) {
  if (n <= 1)
    return 1;
  else
    return n * factorial(n-1);
}
factorial(5)
`); // === 120
```

## How does it work?

This library wraps the [Boa engine](https://github.com/boa-dev/boa) (a JavaScript VM written in Rust) with a thin wrapper in which the Boa JavaScript values are converted to JavaScript values and Boa errors are converted to JavaScript errors.
This library is then compiled to WebAssembly using [wasm-pack](https://github.com/rustwasm/wasm-pack) (`wasm-pack build --target nodejs`) such that you can use it as a sandbox environment to run JavaScript code. The code optimizes for faster execution speed rather than for smaller binary size.

## What does not work?

* ❌ Features that Boa does not support, [see conformance here](https://github.com/boa-dev/boa#conformance)
* ❌ Timeout long-running functions
* ❌ Pass back values from the evaluation other than the aforementioned types
* ❌ Provide anything other than JavaScript source code to the vm
* ❌ Errors being handled safely
  * Errors thrown in the provided source code will be equally converted from Boa JavaScript errors to JavaScript errors, thus you should wrap evaluations in a [try-catch statement](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/try...catch)
* ❌ Never crashing engine
  * Due to the Boa inclusion of tome `todo!` macro's & the project being in development still, the engine may crash for reasons the developers are still working on. For your program safety, you should wrap evaluations in a [try-catch statement](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/try...catch) and reload the engine every time you want to evaluate code (by invalidating the module loader cache), as otherwise, an engine crash may result in the engine not being able to evaluate any further code. A safe example:

```JavaScript
function safeEvaluate(source) {
    delete require.cache[require.resolve("javascript-vm")]
    const vmjs = require("javascript-vm")
    try {
        // evaluate code
        return vmjs.evaluate(source);
    } catch (error) {
        // handle error
    }
}

safeEvaluate(`({x: undefined})`) // [1] error will be caught
safeEvaluate(`() => {}`) // [2] error will be caught

safeEvaluate("true") // === true
safeEvaluate("BigInt(1)") // === 1n
```

What went wrong in the examples above?
* [1] - Boas's JSON serialization does not support `undefined` values.
* [2] - A function was returned.

## Authors

* Aäron Munsters
* 🛠️ Built with [wasm-pack](https://github.com/rustwasm/wasm-pack)
* 🦀 Made possible by [the authors of Boa](https://github.com/boa-dev)