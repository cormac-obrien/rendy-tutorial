# Working with Backends

Rendy can compile renderers for the DirectX 12, Metal, and Vulkan graphics APIs.
You can easily support any combination of these backends by using Cargo features.
Add a features section to your `Cargo.toml` like this:

```toml
[features]
default = ["rendy"]
dx12 = ["rendy/dx12"]
metal = ["rendy/metal"]
vulkan = ["rendy/vulkan"]
```

When you run Cargo, you can pass the `--features` flag to specify which backend to target.
You can use conditional compilation to specify different behavior for different backends.
At the top level of your Rust code, define a `Backend` type
that depends on the backend features you just added:

```rust
#[cfg(feature = "dx12")]
type Backend = rendy::dx12::Backend;
#[cfg(feature = "metal")]
type Backend = rendy::metal::Backend;
#[cfg(feature = "vulkan")]
type Backend = rendy::vulkan::Backend;
```

Now, whenever a backend is required as a type parameter (which is pretty often),
you can just write `Backend` and it'll work no matter which backend you're targeting.
Neat!

If you're not familiar with conditional compilation in Rust,
here's a quick rundown of the attributes you'll be using:

- `#[cfg(feature = "backend")]`
  This is the same attribute you used to define the `Backend` type.
  By naming a feature you defined in your `Cargo.toml`,
  Cargo knows to only compile the following code if `--features=backend` was specified.
- `#[cfg(any(feature = "dx12", feature = "metal", feature = "vulkan"))]`
  This attribute applies to any code that is backend-agnostic,
  which is most of the code you'll write with Rendy.
- `#[cfg(not(any(feature = "dx12", feature = "metal", feature = "vulkan")))]`
  This attribute applies to code that will only compile when none of the backends have been selected.
  It's useful for dummy implementations of types and functions,
  which you might want if you're testing other parts of your software.
  
If you're still confused about conditional compilation,
the [Rust Reference](rust-reference) has more in-depth coverage of the topic.

[rust-reference]: https://doc.rust-lang.org/reference/conditional-compilation.html
