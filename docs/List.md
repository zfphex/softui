Currently the framework has no list widget and no support for containers (vec, array, etc).

Some of the problems I'm running into:
- Typically UI nodes are allocated on a tree then the layout is calculated form these nodes.
- This works well for small amounts of data, however a list can hold potentially millions of items.
    - The layout system must be independent from the root ui tree.
    - This means there needs to be a way to calculate layout without allocating nodes.
    - The implication here is that the nodes are already stored somewhere. 
- Nodes are not widgets and do not implement the associated traits.
    - I wonder if the generic widget could just be turned into a node.
    - The cache system is a little unclear, but it wouldn't be a huge change to add final layout into generic widgets.
- The lifetimes of data are super unclear.
- A list will need to hold any arbitrary combination of widgets (including containers) without allocating into the tree.
    - I'm not sure this is actually possible.
    - This would require moving back to the old method of using linked lists.
    - I guess we could support both?
- Should lists be heterogeneous or homogeneous?
    - A single type would be easier to work with but this is not very flexible.
    - If nesting is removed the layout and allocation structure is much simpler.


