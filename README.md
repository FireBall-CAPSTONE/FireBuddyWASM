
# Fire Buddy Renderer

WebGL renderer written in Rust for CSCI 4155 at UNCC.

The project compiles to Web Assembly and can be easily deployed on an existing website.

Built using:

* [wasm-bindgen](https://docs.rs/wasm-bindgen/latest/wasm_bindgen/)
* [js-sys](https://docs.rs/js-sys/latest/js_sys/)
* [lazy-static](https://docs.rs/lazy_static/latest/lazy_static/)
* [wasm-pack](https://rustwasm.github.io)

## How to compile

Compile project with:

``` bash
wasm-pack build --target web
```

This creates a folder called `pkg` which contains all the files required to run the project on a website.

* `fire_buddy_renderer_bg.wasm` contains the Web Assembly code.
* `fire_buddy_renderer.js` contains JavaScript functions to call the Web Assembly code.
* `fire_buddy_renderer.d.ts` contains TypeScript definitions for use with TypeScript applications.
* `fire_buddy_renderer_bg.wasm.d.ts` contains definitions for initializing the Web Assembly code

## Example Code

``` html
<!-- index.html -->
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <canvas id="canvas" height="600" width="800" style="position: absolute; top:0px; left: 0px; z-index: -10;">

    <script type="module" src="renderer.js"/>
</body>
</html>
```

```js
// renderer.js
import init, {App} from "./pkg/fire_buddy_renderer.js";

let canvas = document.getElementById('canvas');

async function run() {
    // Call init() to load wasm
    await init();

    // Create a new app
    let app = new App('canvas');
    
    // Init support variables
    var lastDrawTime = Date.now();
    var delta = 0.0;

    // define the animation loop
    function render() {

        // Update to update world values
        // In this example, delta is in ms and it must be converted into seconds
        app.update(delta/1000.0);

        // Render to draw to the canvas
        app.render();

        // Update delta
        const currTime = Date.now();
        delta = ((currTime - lastDrawTime));

        // Update lastDrawTime
        lastDrawTime = currTime;

        // Request animation frame to begin next frame
        requestAnimationFrame(render);
    }

    // start animation loop
    render();
}

// run the program
await run();
```
> `.wasm` files are resources that are requested from a webserver. You cannot simply open up the html file in a web browser and have the WASM code work. I recommend using the [live server](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer) extension for vscode users. You can also spin up a local web server with python.
