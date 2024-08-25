Widgets need to know:
- what the parent widget area is, this would be either a container or the viewport
    - used to calculate a widget percentage
    - for example `rect().width(0.5)` needs to know half the parent widget area
- what the current font size is for the container
    - used to calculate the em units
- the current state of all user input (mouse, keyboard, controllers, etc)
    - this should be made atmoic, it's not currently.

