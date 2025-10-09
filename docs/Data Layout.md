- Currently we use a single tree that stores a flat array of nodes.
    -  If a node has children it's stored as an index into the tree.
    - Currently there is no widget information being stored in the tree

- We could use a second flat array that stores `Box<dyn widget>`
    - Widgets can be stored as indexes inside of a node.
    - This would require iterating over two seperate chunks of memory
    - Doesn't really seem like a good idea.
    - Each node could store the widgets instead.
    - In `Vec<Box<dyn T>>` `Box` appears redundant since `Vec` is already heap allocated.

- Widget rules need to be better defined
    - Each widget is created and destoryed each frame.
    - Each widget click handler is executed after sizing and positioning.
        - This way the user can introspect on the state of the widget inside of a closure.
        - There should be a second (non-closure) what to handle click (more akin to typical immediate mode)
    - Widgets should not store large amounts of information (i.e. images, audio, etc.)
        - Instead a widget should refer back to this information.
        - `image(&image_data)`
    - Technically a widget could be read only, but the user might want to read it after layout changes.
