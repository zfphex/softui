Currently widgets cannot hold their own state since the library is immediate mode.
This makes ergonomics quite bad for users trying to create input boxes and buttons.

To fix this, we can store the focused state inside of every node. That way the user doesn't need to do anything to know if it's an active or selected widget.