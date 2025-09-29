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

- Widgets Left

Root
    Group
        Rect(50%, 20)
        Rect(50%, 20)
        SubGroup 
            Rect(50%, 20)
            Rect(50%, 20)
    Group 
        Rect(50%, 20)
        Rect(50%, 20)
        SubGroup 
            Rect(50%, 20)
            Rect(50%, 20)

The groups should get 1/2 of the root width.

The sub groups should get 1/3 of the groups width so 1/6 of the root width.

The root has 2 widgets pending.

The groups have 3 widgets pending.

The subgroups have 2 widgets pending.

We only care about remaining widgets if it's not the last child.

The Rect(50%) should be 50% of the SubGroup, it does not need to calculate the area from the remaining widgets.

Yeah idk...


- Sizing (Requires Two Passes)

TODO: I think I messed up a bit trying to put all the sizing information in the positioning pass.
There should be enough information to calculate the size in a second pass.
This way the code is a bit more fragmented and the x and y position calculations are not lumped together.
I think that writing the position function like that is possible but it's way too complex for me to conceptualise.

Okay but what happens if we have two widgets with 100% Sizing?

v!(rect().w(100.percent()), rect().w(100.percent()))

//Example Assumes LeftRight only
Self.SetSize(Parent)
    TotalWidth = 0
    TotalHeight = 0
    WidgetsToSecondPass = Self.Size.WidgetsToSecondPass.UnwrapOr(1)

    ParentWidth = Parent.Width - Padding * 2
    ParentHeight = Parent.Height - Padding * 2

    TotalWidth += Gap * Self.Children.Len() - 1

    IsSecondPass = Self.Size.WidgetsToSecondPass.IsSome()

    AvailableWidth = ParentWidth
    AvailableHeight = ParentHeight

    If IsSecondPass
        AvailableWidth = (ParentWidth - Self.Size.Width) / Self.Size.WidgetsToSecondPass

        //Not sure what this would be for LeftRight
        //AvailableHeight -= Self.Size.Height

    Child in Self.Children
        //Calculate the size of containers.
        //If it's not a container the Rect passed in will do nothing.
        Child.SetSize((0, 0, AvailableWidth, AvailableHeight))

        WantedSize = Child.WantedSize()

        If Not IsSecondPass And Child.IsContaier()
            If WantedSize.Width | WantedSize.Height == Auto | Percentage
                WidgetsToSecondPass += 1
        Else 
            Match WantedSize.Width
                Percent
                    Child.SetWidth(AvailableWidth * Percent)
                Auto
                    Child.SetWidth(AvailableWidth / WidgetsToSecondPass)

            Match WantedSize.Height
                Percent
                    Child.SetHeight(AvailableHeight * Percent)
                Auto
                    Child.SetHeight(AvailableHeight / WidgetsToSecondPass)

        TotalWidth += Child.Size.Width
        TotalHeight += TotalHeight.Max(Child.Size.Height)
        
    Self.Size = (0, 0, Width, Height)

- Position

Group.Position(Parent = WindowSize)
    X = Parent.X + Padding
    Y = Parent.Y + Padding
    GroupWidth = 0
    GroupHeight = 0

    WidgetsLeft = 1

    //Different for TopBottom direction
    AvailableWidth = Parent.Width - Group.Width / WidgetsLeft = 800 - 20 / 1 = 780
    AvailableHeight = Parent.Height.Max(Group.Height)

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
            GroupWidth += Width
            GroupHeight = GroupHeight.Max(Height) 

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
            GroupWidth += Width
            GroupHeight = GroupHeight.Max(Height) 
        
        Rect.Position(Parent = ???)
            Width = Rect.Width = 20
            Height = Rect.Height = 20

            X += Rect.Size.X
            GroupWidth += Width
            GroupHeight = GroupHeight.Max(Height) 

    Group.Size = (X, Y, GroupWidth, GroupHeight)

The avaliable space to distribute between H1 and H2 is (800 - 20 = 780, 600)
Note the height doesn't change.

