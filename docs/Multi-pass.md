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

Each unsized widget will get the remaining_length / remaining_widgets.

Usable Width = 780 / 2 = 390

H1.size = (78 + 312 = 390, 20)
H2.size = (78 + 312 = 390, 20)

- Position Pass

Just sets the x and y position.

# Implementation

Group H1(Rect, Rect) H2(Rect, Rect) Rect

Here we have a root container and seven widgets.
The two containers have an unknown size.
Their children have an unknown size.
The last widget has a fixed size.

- Inital

Direction LeftRight
WindowSize (800, 600)
Group has a size of (Auto, Auto)
H1 has a size of (Auto, Auto)
    Rect(20%, 20)
    Rect(80%, 20)
H2 has a size of (Auto, Auto)
    Rect(20%, 20)
    Rect(80%, 20)

Rect(20, 20)

- Calculate Size

Group has a size of (20, 20, 1 widget remains)

H1 has a size of (Auto, Auto, 2 widgets remain)
    Rect(20%, 20)
    Rect(80%, 20)

H2 has a size of (Auto, Auto, 2 widgets remain)
    Rect(20%, 20)
    Rect(80%, 20)

- Position

Group.Position(Parent = WindowSize)
    X = Parent.X + Padding
    Y = Parent.Y + Padding
    Width = Parent.Width - Group.Width = 800 - 20 = 780
    Height = Parent.Height.Max(Group.Height) = 20 //can overflow the window

    WidgetsLeft = 1
    //Different for TopBottom direction
    AvailableWidth = Width / WidgetsLeft = 780
    AvailableHeight = Height

        H1.Position(Parent = (AvailableWidth, AvailableHeight))
            Width = Parent.Width - H1.Width = 780 - 0
            Height = Parent.Height.Max(H1.Height) = 20
            WidgetsLeft = 2
            AvailableWidth = Width / WidgetsLeft = 390
            AvailableHeight = Height = 20
                Rect.Size = (20% * AvailableWidth, 20) = (78, 20)
                Rect.Size = (80% * AvailableWidth, 20) = (312, 20)
            
            H1.Size = (78 + 312 = 390, 20)
            X += H1.Size.X

        H2.Position(Parent = (AvailableWidth, AvailableHeight))
            Width = Parent.Width - H1.Width = 780 - 0
            Height = Parent.Height.Max(H1.Height) = 20
            WidgetsLeft = 2
            AvailableWidth = Width / WidgetsLeft = 390
            AvailableHeight = Height = 20
                Rect.Size = (20% * AvailableWidth, 20) = (78, 20)
                Rect.Size = (80% * AvailableWidth, 20) = (312, 20)
            
            H1.Size = (78 + 312 = 390, 20)
            X += H1.Size.X
        
        Rect.Position(Parent = ???)
            Width = Rect.Width = 20
            Height = Rect.Height = 20
            X += Rect.Size.X

The avaliable space to distribute between H1 and H2 is (800 - 20 = 780, 600)
Note the height doesn't change.

