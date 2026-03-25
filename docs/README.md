### Context

- Initalises the window and framebuffer.
- `ctx.event()` handles all the input and window events from the os.
- Draws primatives (rectangles, circles, text, image, svg, etc.) into the framebuffer.
- Stores a list of "commands". A primative and an area (location to draw at).
  - There is no depth in the draw process, so it uses the draw order.
  - The background is synced to the monitor refresh rate (on windows).
  - The background is filled using a weird extra method (`ctx.set_fill_color(..)`)
  - Previously this was a global lock free queue. It was thread safe but not really useful.
- `draw_frame` takes all the commands and calls the correct primative draw function.
  - Primatives are not flexible and users cannot modify them.
  - This could be changed be a `Primative` trait, queue could be `Vec<dyn Primative>` then just call draw_primative on each.
  - Not necessary at the moment.
- `draw_layout` set the root node size and recursively loop through all of the layout nodes and draw them.
  - The lifetimes are a bit broken since the root node has a lifetime of `<'a>` (it last a single frame or as long as the main function)
    and the tree has a lifetime of `<'static>`, it lasts the whole lifetime of the program.

### Tree

- Stores a flat array of nodes which are iterated through and layed out each frame.
- Most of the tree code does essentially nothing and is just boilerplate for taffy.
- Node
  - Contains layout information, a list of indicies to children in the tree and a pointer to a dyn widget.
  - Can only be 4 types of node, Flex, Fit, Grid, Text. Each node type is rendered differently during layout.
    - There is a weird disconnect between nodes and widgets that's not easy to conceptualise.
- `draw_tree`
  - Recursive over each of the nodes
- `compute_child_layout`
  - calls a taffy function `compute_child_layout` which takes in a user defined closure
  - gets the node from the tree and computes either a flexbox, no layout (hidden) or leaf node (no children).
  - It's unclear what exactly taffy does here but it's not really important.
  - I tried to write a layout engine and all the tests in the world couldn't save me from the conceptual nightmare of that problem.

### Widget Trait

- 