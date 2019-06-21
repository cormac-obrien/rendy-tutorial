# Open a Window

As mentioned in the last section, Rendy uses `winit` to handle display and input.
This section covers window creation and event handling in `winit`.
If you're already familiar with this library, feel free to move on to the next section.

To create a window in `winit`, you need two things:
an `EventsLoop`, which handles input like mouse clicks and keystrokes,
and a `WindowBuilder`, which specifies the attributes you want and creates the window.
Creating an `EventsLoop` is very simple:

```rust
use rendy::wsi::winit::EventsLoop;

let mut events_loop = EventsLoop::new();
```

The `WindowBuilder` allows you to specify the title, size, icon
and other customizable attributes of the window.

```rust
use rendy::wsi::winit::WindowBuilder;

let window = WindowBuilder::new()
    .with_title("Hello, Rendy!")
    .with_dimensions((800, 600).into())
    .build(&events_loop)
    .expect("Window creation failed.");
```

*Note: that `.into()` in the `.with_dimensions()` call is just a convenience
to convert a simple pair of integers into a `winit::LogicalSize`.
You can write out the full initializer for that type if you want to.*

You should also add a bit of input handling so that hitting the close button works properly:

```rust
use rendy::wsi::winit::{Event, WindowEvent};

let mut close = false;
while !close {
    events_loop.poll_events(|event| {
        match event {
            Event::WindowEvent { event: w, .. } => match w {
                WindowEvent::CloseRequested => close = true,
                _ => (),
            }
            _ => (),
        }
    });
}
```

`events_loop.poll_events()` runs the closure you provide on every event in the queue.
Now your program will run forever until you close the window. Try running it now!

Depending on what operating system you use, the contents of the window might vary.
On some systems it might be white or black,
while on others it might have garbage data or look transparent.
Don't panic if it looks weird -- that's just because you haven't drawn anything yet.
