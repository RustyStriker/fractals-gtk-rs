# Fractals - A GTK app made in rust

Some simple fractals made in rust and gtk.

Just a note, framerate drops when there is a large amount of shapes on the screen, because I could not find any way to make it render in the background(as all of the gtk drawing buffers and surfaces are not thread safe)

## Current fractals

- Simple circles
- Simple lines
- Some weird meshup of the circles and lines
- Tree
- Random tree
