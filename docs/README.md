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