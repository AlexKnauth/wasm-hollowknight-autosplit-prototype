# wasm-hollowknight-autosplit

An auto splitter for Hollow knight.

## Compilation

This auto splitter is written in Rust. In order to compile it, you need to
install the Rust compiler: [Install Rust](https://www.rust-lang.org/tools/install).

Afterwards install the WebAssembly target:
```sh
rustup target add wasm32-unknown-unknown --toolchain stable
```

The auto splitter can now be compiled:
```sh
cargo b
```

The auto splitter is then available at:
```
target/wasm32-unknown-unknown/release/wasm_hollowknight_autosplit.wasm
```

Make sure too look into the [API documentation](https://livesplit.org/asr/asr/) for the `asr` crate.

You can use the [debugger](https://github.com/CryZe/asr-debugger) while
developing the auto splitter to more easily see the log messages, statistics,
dump memory and more.

## Instructions for livesplit-one-desktop

Clone `livesplit-one-desktop` from https://github.com/CryZe/livesplit-one-desktop

In the `livesplit-one-desktop` repository, modify the `config.yaml` file so that it contains
```yaml
general:
  splits: <path-to-splits.lss>
  auto-splitter: <path-to-wasm_hollowknight_autosplit.wasm>
```
where you replace `<path-to-splits.lss>` with the path to your splits file, and you replace `<path-to-wasm_hollowknight_autosplit.wasm>` with a path to the compiled `wasm` file found at `target/wasm32-unknown-unknown/release/wasm_hollowknight_autosplit.wasm` of this repository.

If you're running anything other than the specific placeholder splits in the `src/splits.json` file of this repository, you should modify that file to have the splits you want, in the order you want, and then re-compile this repository with
```sh
cargo b
```

When you run either `livesplit-one-desktop` or the `asr-debugger`, it needs to have permission to read memory of other processes.
On Mac, that might require running it under `sudo`.
For example in the `livesplit-one-desktop` repository, you can run
```sh
sudo cargo run --release
```

Finally, do not manually split, skip, or undo splits while running with this autosplitter.
The autosplitter will not know that you did that, and the autosplitter's state will be out of sync with `livesplit-one-desktop`'s state.

The keyboard shortcuts of `livesplit-one-desktop` assume the Qwerty keyboard layout,
so you may need to press where the key would be if you were using Qwerty.
For example to save splits is "Control S" on Qwerty, but on a Dvorak keyboard,
the Qwerty key "S" is where the Dvorak key "O" is, so use Dvorak "Control O" to save instead.

## Instructions for obs-livesplit-one

On Mac, it only works with OBS 28, not 29. You can get OBS 28 from the OBS releases page if needed. [`OBS Studio 28.1.2`](https://github.com/obsproject/obs-studio/releases/tag/28.1.2) is the latest version that works with this autosplitter on Mac.

It only has a possibility of working with obs-livesplit-one release v0.3.2 or later.
Go to the [obs-livesplit-one releases](https://github.com/LiveSplit/obs-livesplit-one/releases) page,
and under the `Assets` section, download the one that matches your architecture and operating system.
While following the instructions in the [`How to install`](https://github.com/LiveSplit/obs-livesplit-one/blob/master/README.md#how-to-install) section of the obs-livesplit-one README file,
make sure you install it into your OBS 28 installation, not any other OBS version you might have.

If you're using Steam on Mac, you need a Hollow Knight installation separate from your steam installation.
You can create a separate installation like this:

1. In the Steam Library entry, Game settings, Properties, Installed Files, Verify integrity of game files.
2. Browse local files, copy the `hollow_knight.app` into a new location, and add `steam_appid.txt` files in that new location. I don't know exactly where `steam_appid.txt` belongs, so I just put it in many places both next to and inside the package contents of the `hollow_knight.app` in the new location.
3. In obs-livesplit-one properties, set the Game path to that new location `hollow_knight.app`.

If setting the Game path to a path to `hollow_knight.app` doesn't work, try setting it to a path to `hollow_knight.app/Contents/MacOS/Hollow Knight` and see if that does any better.

If it still isn't working, another thing to try on Mac could be running OBS under `sudo`.
For example in the directory where you have OBS 28 installed, you can run
```sh
sudo ./OBS.app/Contents/MacOS/OBS
```

But even while trying all of this, I still haven't managed to get it working yet.
