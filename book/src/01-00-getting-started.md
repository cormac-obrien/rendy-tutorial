# Getting Started

## In this Chapter

In this chapter, you'll learn:
- How to set up your Rust project to use Rendy
- How to open the window you'll render to
- How to handle Rendy's multiple backends
- How to define a graphics pipeline
- How to build a renderer from your pipeline

## Rendy Initialization

Our next step is to get our graphics backend up and running.

```rust
#[cfg(any(feature = "dx12", feature = "metal", feature = "vulkan"))]
fn main() {
    /// ...
}
```

And create a second "dummy" `main()` function to run when the features aren't enabled:

```rust
#[cfg(not(any(feature = "dx12", feature = "metal", feature = "vulkan")))]
fn main() {
    println!("Please enable one of the backend features: dx12, metal, vulkan");
}
```

### Create a Factory

To initialize the backend, we need to create a `Factory`.
This object interfaces with the graphics device, creating and managing the resources you'll need.
Rendy provides a `Config` type which specifies how the graphics backend is set up,
but for now, we can use the default configuration:

```rust
use rendy::factory::{Config, Factory};

let config: Config = Default::default();
let (mut factory, mut families): (Factory<Backend>, _) =
    rendy::factory::init(config).expect("Factory creation failed.");
```

The default `Config` picks the first available graphics device
(typically a dedicated GPU if you have one),
initializes the built-in memory allocator with reasonable values,
and retrieves a single graphics queue family from the chosen device.

### Set Up a Render Pipeline
