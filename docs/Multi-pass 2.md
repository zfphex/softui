Unit types:

- Fixed Pixel
- Parent Relative Percentage
- Fill available space
- Fit child size

```js
Window (500, 600)

Root (Fill, Fill)
    Rect(250, 100)
    H1 (Fill, Fill)
        Rect(20%, 300)
        Rect(80%, 400)
    H2 (Fill, Fill)
        Rect(20%, 500)
        Rect(80%, 600)
```

H1 & H2 should get 250 px of width split between both.

- 20% of 250 is 50
- 80% of 250 is 200

H1 (250, 400)
H2 (250, 600)

- Sizing

Root(Fill, Fill)
Size(500, 600) fill parent (window)

Rect(250, 100) fixed so size is static

H1(Fit, Fit) and H2(Fit, Fit) size is based on children
When multiple containers are Fill, space is evenly distrubuted across all containers.

<!-- This doesn't really make sense...  -->

So H1 and H2 have a maximum width of (500 - 250) / 2 = 125
and a maximum height of (600 - 100) / 2 = 250

Rect(20%, 300) 20% of parent width 125 _ 0.2 = 25
Rect(80%, 300) 80% of parent width 125 _ 0.8 = 100

Since H1 is horizontal it's size is (100 + 25, 400)

H2 is the same as H1

- Sizing Implementation (Assuming Left to Right layout)

```
fn size(parent)
    total_width = gap * self.children.len() - 1
    total_height = 0

    parent_width = parent.width - self.padding * 2
    parent_height = parent.height - self.padding * 2

    rem_width = parent.width
    rem_height = parent.height

    for child in self.children {
        size = child.size(0, 0, rem_width, rem_height)

        total_width += size.width
        total_height = total_height.max(size.height)
    }

    return (0, 0, total_width, total_height)
```

- Fill Cases

```
Parent(100, 100)
    Rect(50, 50)
    H1(Fill, Fill)
        Rect(50%, 50%)
    H2(Fill, Fill)
```

Rect = (50, 50)
H1 = (25, 25)
Rect = (12.5, 12.5)
H2 = (25, 25)

- Nested Cases

```
Parent(100, 100)
    H1(50%, 50%)
        Rect(50%, 50%)
    H2(100%, 100%)

H1 = (50, 50)
Rect = (25, 25)
H2 = (100, 100) //Overflow
```

```
Parent(100, 100)
    H1(50%, 50%)
        H2(50%, 50%)

H1 = (50, 50)
    H2 = (25, 25)
```

- Fit Cases

```
Parent(100, 100)
    Group(Fill)
        Group(Fit)
            Rect(10, 10)

Group = (100, 100)
    Group = (10, 10)
        Rect = (10, 10)
```

- Invalid Cases

```
H1(Fit)  //  needs to know children size
    Rect(50%) //  needs to know parent size
```

```
H1(Fit)        //  needs to know children size
    Rect(Fill) //  needs to know parent size
```
