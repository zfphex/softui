The main things holding back this library are:
- No native varidics
- No compile time type reflection
- No way to do defered instantiation

What is defered instantiation.

Consider the following:

```rs
button().x(20).y(30)
button().draw()

Button {
    area: {x: 0, y: 0, width: 0, height: 0}
}

//.x(20)
Button {
    area: {x: 20, y: 0, width: 0, height: 0}
}

//.y(20)
Button {
    area: {x: 0, y: 20, width: 0, height: 0}
}

//.draw()
draw(Button {
    area: {x: 0, y: 20, width: 0, height: 0}
})
```

Here the order of execution is very clear. Each function is called one after the other.

However, what if we wanted to defer the execution of some code until the user has finished modifying it.

```rs
// .draw() defered here.
// V
button().x(20).y(30)
//                   ^ button().draw() called 
```

The `Drop` trait is quite simliar but `Drop` is called at the end of scope and not at the end of instantiation.

Why does this matter?


```rs
v(button(), button())
```