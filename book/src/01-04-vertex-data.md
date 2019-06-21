# Vertex Data

You can't draw anything without having some data to feed your renderer.
For this first example, you'll be rendering the multicolor triangle
that all graphics programmers know and love!

For this task, you'll need a vertex type with a position attribute and a color attribute.
Conveniently, Rendy provides this (and several other vertex types) in its `mesh` submodule
as the `PosColor` type.
Define the triangle like this:

```rust
use rendy::mesh::{Color, PosColor, Position};

const VERTEX_DATA: [PosColor; 3] = [
    PosColor {
        position: Position([-0.5, 0.5, 0.0]),
        color: Color([1.0, 0.0, 0.0, 1.0]),
    },
    PosColor {
        position: Position([0.0, -0.5, 0.0]),
        color: Color([0.0, 1.0, 0.0, 1.0]),
    },
    PosColor {
        position: Position([0.5, 0.5, 0.0]),
        color: Color([0.0, 0.0, 1.0, 1.0]),
    },
];
```

The `PosColor` type implements a Rendy trait called `AsVertex`,
which provides vertex format information to a graphics pipeline.
This makes implementors of the trait easy to use without needing to specify
stride and attribute sizes every time you use the vertex format.
You'll see how to implement your own `AsVertex` types in a later section.
