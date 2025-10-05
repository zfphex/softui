```rust
pub struct Tree {
    pub nodes: Vec<Node>,
    pub widgets: Vec<Box<dyn Widget>>,
}

pub struct Image {
    pub image_data: &[u8],
    pub size: [Unit; 2],
    // Required elements for layout but not required for user to implement.
    // These are in size struct.
    // size: [f32; 2],
    // pos: [f32; 2],
    // padding: f32,
    // margin: f32,
    // style: Style,
    // Direction and gap are not required since they are container only...
}

impl Widget for Image {
    fn desired_size(&self) -> [Unit; 2] {
        self.size
    }
}

pub struct Node {
    pub desired_size: [Unit; 2],
    pub min_size: [Unit; 2],
    pub max_size: [Unit; 2],
    pub size: [f32; 2],
    pub pos: [f32; 2],
    pub padding: [f32; 4],
    pub margin: [f32; 4],
    pub direction: Direction,
    pub gap: f32,
    //Background color and foreground color properties.
    pub style: Option<Style>,
    //A node can point to a widget.
    pub widget: Option<usize>,
    pub children: Vec<usize>,
}

let tree = Tree::new()
let image = image("test.png").max_w(75.percent())
let widget_idx = tree.widgets.add(image)
let node_idx = tree.nodes.add(widget_idx); //An image can be a container now???

let text = text("Text inside an image?")
let widget_idx = tree.widgets.add(text)
let child_idx = tree.nodes.add(widget_idx)

//Add the text as a child to the
tree.nodes.add_child(node_idx, child_idx)
```

Currently the whole system uses a single node struct.
Previously the system was basically just a linked list.
Pointers to groups where nested inside of other groups and styling and input was chained ontop using the type system.
Not only was this slow and difficult to debug, but any change to the base api required fixed the "monads" stacked on top.

Node is just an empty type that stores layout information, like size, direction, gap, children.
It's still not ideal since every child requires allocating memory.
I think that an arena would just be too complicated to implement, it's already a mess keeping track of everything.

I think the weird thing about decoupling the widgets and layout nodes, is that you could reference the same widget in different nodes really easily.

If there is a single widgets vector, the lifetime of every widget is always <'a>.

- Tree (holds all nodes)
  - Node (a single node in the tree)
    - Container (just Vec<Node> but allows user to change direction, padding, gap, etc.)
      - Generic Widget (Not sure about this yet, it's basically a node but contains a widget?)
      - Image (Basic widget)
      - Rect (Basic widget)

User creates a rectangle.

```rs
flex!(
    v!(
        rect().wh(20)
    )
)


let rect = rect().wh(20)
//This will not work with click handlers?
let container = vec![rect.into_node()]; 
let root = Tree::new();
root.add(container);

```
