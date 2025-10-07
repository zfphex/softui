Consider the macro

```rs
v!(rect(), rect())
```

Since there is no context pass in it must either return something or mutate global state.

_I think I have no choice but to throw away multi-threadeding, we can come back to this later._

- What should a macro return
    - Widget ID (usize)
        - Has global state 
        - Use areana allocator.
    - `Vec<Box<dyn Widget>>`
        - Tree is stored as linked list 

- How do we want to store widgets?
    - Flat array is best
    - `Vec<Box<dyn Widget>>`
    - Store tree as indexes + nodes.
    - `Vec<Node>`

- How do we want to store large data (images)?
    - Keep in memory, don't load every frame.
    - Load at startup/when called?
    - `image("test.png") -> image_ref(image)`

- How often do we we to regenerate widgets?
    - Every frame (use arena allocator)
    - Retained (idk)



