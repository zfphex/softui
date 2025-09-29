Unit types:

- Fixed Pixel
- Parent Relative Percentage
- Fill available space
- Fit child size

```js
Window (500, 600)

Root (Fill, Fill)
    Rect(250, 100)
    H1 (Fit, Fit)
        Rect(20%, 300)
        Rect(80%, 400)
    H2 (Fit, Fit)
        Rect(20%, 500)
        Rect(80%, 600)
```

H1 & H2 should get 250 px of width split between both.

- 20% of 250 is 50
- 80% of 250 is 200

H1 (250, 400)
H2 (250, 600)

- Sizing Pass

Root(Fill, Fill)
Size(500, 600) fill parent (window)

Rect(250, 100) fixed so size is static

H1(Fit, Fit) and H2(Fit, Fit) size is based on children
Fill and Fit have the space allocation rules, they are evenly distrubuted across all containers.

<!-- This doesn't really make sense...  -->

So H1 and H2 have a maximum width of (500 - 250) / 2 = 125
and a maximum height of (600 - 100) / 2 = 250

Rect(20%, 300) 20% of parent width 125 _ 0.2 = 25
Rect(80%, 300) 80% of parent width 125 _ 0.8 = 100

Since H1 is horizontal it's size is (100 + 25, 400)

H2 is the same as H1
