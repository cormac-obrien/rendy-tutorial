# Project Setup

Setting up a project to use Rendy is easy!
First, add `rendy` to the `[dependencies]` section of your `Cargo.toml`:

```toml
[dependencies]
rendy = "0.3.1"
```

Rendy also relies on a few external libraries:

- [`winit`](winit), which handles windowing systems and input.
- [`gfx-hal`](gfx-hal), the lower-level graphics API that powers Rendy.
- [`failure`](failure), which provides more convenient error types than the standard library.

`winit` is the easiest of these to include, since Rendy re-exposes it for you.
Anytime you need to `use` something from `winit`, you can get it from `rendy::wsi::winit`.

You do need to add `gfx-hal` and `failure` to your `Cargo.toml`, so go ahead and do that now:

```toml
[dependencies]
# ...
gfx-hal = "0.2"
failure = "0.1"
```

Great! You're all set to start building your first renderer.

[winit]: https://crates.io/crates/winit
[gfx-hal]: https://crates.io/crates/gfx-hal
[failure]: https://crates.io/crates/failure
