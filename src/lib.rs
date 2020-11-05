extern crate gtk;
extern crate cairo;
use gtk::prelude::*;
use cairo::Context as Cairo;

pub fn draw_circle(cairo : &Cairo, x : f64, y : f64, radius : f64){
    cairo.set_line_width(radius / 10.0);
    cairo.arc(x,y,radius,0.0,360.0);
    cairo.stroke();
}

pub fn circle_fractals(cairo : &Cairo, size : (f64,f64), start : f64, min : f64){
    circle_fractals_helper(cairo, size.0 / 2.0, size.1 / 2.0, start, min)
}

fn circle_fractals_helper(cairo : &Cairo, position : f64, y : f64, length : f64, min : f64){
    let new_len = length / 2.0;
    if new_len > min{
        circle_fractals_helper(cairo, position + length, y, new_len, min);
        circle_fractals_helper(cairo, position - length, y, new_len, min);
        
        circle_fractals_helper(cairo, position, y + length, new_len, min);
        circle_fractals_helper(cairo, position, y - length, new_len, min);
    }
    cairo.set_source_rgb(0.0, 1.0 - (min / length), 0.0);
    draw_circle(cairo, position, y, length);


}