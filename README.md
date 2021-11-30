# Fractal clock wallpaper

This is a simple refactor of [the egui demo lib](https://github.com/emilk/egui/tree/master/egui_demo_lib) to only keep
the fractal clock code for use with [Wallpaper Engine](https://www.wallpaperengine.io/en). The build step encodes the
built wasm file in base64, to then embed it in the html index page. This is necessary due to the fact Wallpaper Engine
does not start a local server, and thus there is no way to bypass the safeties in place that prevent loading the wasm
file as an external resource. Also, all settings can be changed as user properties from the wallpaper engine settings
menu.

# Build

The project uses [cargo make](https://github.com/sagiegurari/cargo-make). Simply run:

```
cargo make build-opt
```

to create the required output files: the `pkg` folder and the `index.html` file.

# Run

You can simply open the `index.html` file, no local server is required to visualize the result. The only requirement is
to have the `pkg` folder with the `fractal_wallpaper.js` file in it, because the path is hardcoded in the `index.html`
file.

# Credits

Basically all the implementation is taken from the `egui` example referred at the start of the readme, thank them for
their amazing work with the [`egui`](https://github.com/emilk/egui) library!