What should we do when the user asks for an automatically growing container with percentages?

```rs
//Assume (800w, 600h) window.
flex!(
    h!(rect().w(20.percent()).h(20), rect().w(80.percent()).h(20)),
    h!(rect().w(20.percent()).h(20), rect().w(80.percent()).h(20)),
    rect().wh(20)
).direction(LeftRight)
```

What is the width of the first horizontal container?
Should it be 800 since that's 20% + 80%

Since the direction is LeftRight, the last rect will be pushed outside of the screen.

The First Layout pass should see the h!() is set to Auto and that the children have relative sizing and skip it.
then the remaining space will be reduced by 20 since rect() has fixed size.
The remaing space will now be 780 and a second pass can layout the percentages to use the remaining space.

I think that size should be updated to support

- Remaining Width
- Allocated Width
- Remaining Height
- Allocated Height

The UI library has three stages

- Sizing
- Positioning
- Rendering

The first stage will not be completed since it's unable to tell what the size will be.

- Sizing Pass

Direction = LeftRight
H1.size = (0, 0)
H2.size = (0, 0)
Remaining.size (800, 600)
Rect.size = (20, 20)
Remaining.size (780, 600) //Not sure what to do about the height

- Post Sizing Pass

<!-- The size of H will need to be calculated again.
Rect(156, 20), Rect(624, 20)
H.size = (780, 20) -->

- Layout Pass

TODO
