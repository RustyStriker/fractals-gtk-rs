extern crate gtk;
extern crate cairo;

use cairo::Context as Cairo;
use std::sync::{Weak,Mutex};
use std::thread;
use rand;

pub fn circle_fractals(circles : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, size : (f64,f64), start : f64, min : f64){
    {
        let circles = match circles.upgrade() {
            Some(c) => c,
            _ => return (),
        };
        let mut mutex = circles.lock().unwrap();
        mutex.clear();

    }
    thread::spawn(move || {
        circle_fractals_helper(circles, size.0 / 2.0, size.1 / 2.0, start, min, size)
    });
}

fn circle_fractals_helper(circles : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, position : f64, y : f64, length : f64, min : f64, size : (f64,f64)){
    {
        let color = (1.0 - (y / size.1), 1.0 - (min / length), 1.0 - (position / size.0));
        let circle = Circle{
            position : (position,y),
            radius : length,
            color,
            thickness : length / 10.0,
        };
        thread::sleep(std::time::Duration::from_millis(10));
        let upgrade = match circles.upgrade() {
            Some(u) => u,
            _ => return (),
        };
        let mut mutex = match upgrade.lock() {
            Ok(lock) => lock,
            _ => return (),
        };
        mutex.push(Box::new(circle));
    }
    let new_len = length / 2.0;
    if new_len > min{
        circle_fractals_helper(circles.clone(), position + length, y, new_len, min, size);
        circle_fractals_helper(circles.clone(), position - length, y, new_len, min, size);
        
        circle_fractals_helper(circles.clone(), position, y + length, new_len, min, size);
        circle_fractals_helper(circles.clone(), position, y - length, new_len, min, size);
    }
    
}

pub fn lines_fractal(shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, size : (f64,f64), start : f64, min : f64){
    let from = (size.0 / 2.0, size.1 / 2.0);
    
    // Clear the mutex 
    {
        let shapes = match shapes.upgrade() {
            Some(c) => c,
            _ => return (),
        };
        let mut mutex = shapes.lock().unwrap();
        mutex.clear();

    }

    thread::spawn(move ||{
        lines_fractal_helper(shapes.clone(), from, (from.0 + start, from.1), start, min, size);
        lines_fractal_helper(shapes.clone(), from, (from.0 - start, from.1), start, min, size);
    });
}

fn lines_fractal_helper(shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, from : (f64,f64), to : (f64,f64), len : f64, min : f64, size : (f64,f64)) {
    {
        let color = (1.0 - (from.1 / size.1), 1.0 - (min / len), 1.0 - (from.0 / size.0));
        let line = Line{
            from,
            to,
            color,
            thickness : len / 10.0,
        };
        thread::sleep(std::time::Duration::from_millis(1));
        let upgrade = match shapes.upgrade() {
            Some(u) => u,
            _ => return (),
        };
        let mut mutex = match upgrade.lock() {
            Ok(lock) => lock,
            _ => return (),
        };
        mutex.push(Box::new(line));
    }

    let new_len = len * 0.7;
    if new_len > min {
        let dir = (sign(to.0 - from.0), sign(to.1 - from.1));
        
        lines_fractal_helper(shapes.clone(), to, (to.0 + dir.1 * new_len, to.1 + dir.0 * new_len), new_len, min, size);
        lines_fractal_helper(shapes.clone(), to, (to.0 - dir.1 * new_len, to.1 - dir.0 * new_len), new_len, min, size);
    }
    
}

fn sign(num : f64) -> f64{
    if num == 0.0{
        0.0
    }
    else if num > 0.0 {
        1.0
    }
    else {
        -1.0
    }
}

pub fn line_circle_fractals(shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, size : (f64,f64), start : f64, min : f64){
    // Clear the mutex 
    {
        let shapes = match shapes.upgrade() {
            Some(c) => c,
            _ => return (),
        };
        let mut mutex = shapes.lock().unwrap();
        mutex.clear();

    }
    thread::spawn(move ||{
        lines_circle_fractals_c_helper(shapes.clone(), (size.0 / 2.0,size.1/2.0), start, min, size);
    });
}

fn lines_circle_fractals_c_helper(shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, position : (f64,f64), length : f64, min : f64, size : (f64,f64)){
    let new_len = length * 0.7;
    if new_len > min {
        let side_1 = (position.0 + length,position.1);
        let side_2 = (position.0 - length, position.1);
        let side_3 = (position.0, position.1 + length);
        let side_4 = (position.0, position.1 - length);

        lines_circle_fractals_l_helper(shapes.clone(), side_1, (side_1.0 + new_len,side_1.1), new_len, min, size);
        lines_circle_fractals_l_helper(shapes.clone(), side_2, (side_2.0 - new_len,side_2.1), new_len, min, size);
        lines_circle_fractals_l_helper(shapes.clone(), side_3, (side_3.0 ,side_3.1 + new_len), new_len, min, size);
        lines_circle_fractals_l_helper(shapes.clone(), side_4, (side_4.0 ,side_4.1 - new_len), new_len, min, size);
    }
    let color = (1.0 - (position.1 / size.1), 1.0 - (min / length), 1.0 - (position.0 / size.0));
    let circle = Circle{
        position,
        radius : length,
        color,
        thickness : length / 10.0,
    };
    thread::sleep(std::time::Duration::from_millis(1));
    let upgrade = match shapes.upgrade() {
        Some(u) => u,
        _ => return (),
    };
    let mut mutex = match upgrade.lock() {
        Ok(lock) => lock,
        _ => return (),
    };
    mutex.push(Box::new(circle));
}

fn lines_circle_fractals_l_helper(shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, from : (f64,f64), to : (f64,f64), len : f64, min : f64, size : (f64,f64)){
    let new_len = len * 0.7 ;// 2.0;
    if new_len > min{
        lines_circle_fractals_c_helper(shapes.clone(), to, new_len, min, size);
    }
    let color = (1.0 - (from.1 / size.1), 1.0 - (min / len), 1.0 - (from.0 / size.0));
    let line = Line{
        from,
        to,
        color,
        thickness : len / 10.0,
    };
    thread::sleep(std::time::Duration::from_millis(1));
    let upgrade = match shapes.upgrade() {
        Some(u) => u,
        _ => return (),
    };
    let mut mutex = match upgrade.lock() {
        Ok(lock) => lock,
        _ => return (),
    };
    mutex.push(Box::new(line));
}

pub fn tree_fractal(shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, size : (f64,f64), start : f64, branches : i32){
    // Clear the mutex 
    {
        let shapes = match shapes.upgrade() {
            Some(c) => c,
            _ => return (),
        };
        let mut mutex = shapes.lock().unwrap();
        mutex.clear();

    }
    thread::spawn(move ||{
        tree_fractal_helper(shapes.clone(), (size.0 / 2.0, size.1), (size.0 / 2.0, size.1 - start), branches, size);
    });
}

fn tree_fractal_helper(shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, from : (f64,f64), to : (f64,f64), branches : i32, size : (f64,f64)){
    {
        let color = (1.0 - (from.1 / size.1), branches as f64 / 255.0, 1.0 - (from.0 / size.0));
        let line = Line{
            from, to, color,
            thickness : 2.0,        
        };
        thread::sleep(std::time::Duration::from_millis(1));
        let upgrade = match shapes.upgrade() {
            Some(u) => u,
            _ => return (),
        };
        let mut mutex = match upgrade.lock() {
            Ok(lock) => lock,
            _ => return (),
        };
        mutex.push(Box::new(line));
    }
    //println!("tree from {:?} to {:?} branches left {}",from,to,branches);
    if branches > 0{
        let len = ((to.0 - from.0).powi(2) + (to.1 - from.1).powi(2)).sqrt();
        //let angle = ((to.1 - from.1) / len).asin(); 
        //let angle = if (to.0 - from.0) < 0.0 { angle - 0.5 * 3.14 } else {angle};
        let angle = (to.1 - from.1).atan2(to.0 - from.0);
        let len = len * 0.7;

        let to_1 = in_angle(len, -3.14 * 0.1 + angle, to);
        let to_2 = in_angle(len, 3.14 * 0.1 + angle, to);
        let to_4 = in_angle(len, -3.14 * 0.3 + angle, to);
        let to_5 = in_angle(len, 3.14 * 0.3 + angle, to);

        tree_fractal_helper(shapes.clone(), to, to_1, branches - 1, size);
        tree_fractal_helper(shapes.clone(), to, to_2, branches - 1, size);
        //tree_fractal_helper(cairo, to, to_3, branches - 1, size);
        tree_fractal_helper(shapes.clone(), to, to_4, branches - 1, size);
        tree_fractal_helper(shapes.clone(), to, to_5, branches - 1, size);

    }
    
}

pub fn random_tree_fractal(shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, size : (f64,f64), start : f64, branches : i32){
    // Clear the mutex 
    {
        let shapes = match shapes.upgrade() {
            Some(c) => c,
            _ => return (),
        };
        let mut mutex = shapes.lock().unwrap();
        mutex.clear();

    }
    thread::spawn(move ||{
        random_tree_fractal_helper(shapes.clone(), (size.0 / 2.0, size.1), (size.0 / 2.0, size.1 - start), branches, size);
    });
}

fn random_tree_fractal_helper(shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, from : (f64,f64), to : (f64,f64), branches : i32, size : (f64,f64)){
    {
        let color = (1.0 - (from.1 / size.1), branches as f64 / 255.0, 1.0 - (from.0 / size.0));
        let line = Line{
            from, to, color,
            thickness : 2.0,        
        };
        thread::sleep(std::time::Duration::from_millis(1));
        let upgrade = match shapes.upgrade() {
            Some(u) => u,
            _ => return (),
        };
        let mut mutex = match upgrade.lock() {
            Ok(lock) => lock,
            _ => return (),
        };
        mutex.push(Box::new(line));
    }
    //println!("tree from {:?} to {:?} branches left {}",from,to,branches);
    if branches > 0{
        let len = ((to.0 - from.0).powi(2) + (to.1 - from.1).powi(2)).sqrt();
        //let angle = ((to.1 - from.1) / len).asin(); 
        //let angle = if (to.0 - from.0) < 0.0 { angle - 0.5 * 3.14 } else {angle};
        let angle = (to.1 - from.1).atan2(to.0 - from.0);
        let len = len * 0.7;

        let mut rand_range = rand::thread_rng();
        let angle_1 = rand::Rng::gen_range(&mut rand_range,0.0, 0.5);
        let angle_2 = rand::Rng::gen_range(&mut rand_range,0.0,0.5);

        let to_1 = in_angle(len, -3.14 * angle_1 + angle, to);
        let to_2 = in_angle(len, 3.14 * angle_1 + angle, to);
        let to_4 = in_angle(len, -3.14 * angle_2 + angle, to);
        let to_5 = in_angle(len, 3.14 * angle_2 + angle, to);

        for _ in 0..2{
            let random = rand::random::<u8>() % 0b1111;
            if random & 0b0001 != 0 {
                random_tree_fractal_helper(shapes.clone(), to, to_1, branches - 1, size);
            }
            if random & 0b0010 != 0 {
                random_tree_fractal_helper(shapes.clone(), to, to_2, branches - 1, size);
            }
            if random & 0b0100 != 0 {
                random_tree_fractal_helper(shapes.clone(), to, to_4, branches - 1, size);
            }
            if random & 0b1000 != 0 {
                random_tree_fractal_helper(shapes.clone(), to, to_5, branches - 1, size);
            }
        }
    }
}

fn in_angle(len : f64, angle : f64, from : (f64,f64)) -> (f64,f64){
    (len * angle.cos() + from.0, len * angle.sin() + from.1)
}

pub trait Shape {
    fn draw(&self, cairo : &Cairo);
}

#[derive(Debug, Clone)]
pub struct Circle {
    position : (f64,f64),
    radius : f64,
    color : (f64,f64,f64),
    thickness : f64,
}
impl Circle {
    pub fn new(position : (f64,f64), radius : f64) -> Circle{
        Circle{
            position,
            radius,
            color : (1.0,1.0,1.0),
            thickness : 1.0,
        }
    }
}
impl Shape for Circle{
    fn draw(&self, cairo : &Cairo){
        cairo.set_source_rgb(self.color.0,self.color.1,self.color.2);
        cairo.set_line_width(self.thickness);
        cairo.arc(self.position.0,self.position.1,self.radius,0.0,7.0);
        cairo.stroke();
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    from : (f64,f64),
    to : (f64,f64),
    color : (f64,f64,f64),
    thickness : f64,
}
impl Shape for Line{
    fn draw(&self, cairo : &Cairo){
        cairo.set_source_rgb(self.color.0,self.color.1,self.color.2);
        cairo.set_line_width(self.thickness);
        cairo.move_to(self.from.0, self.from.1);
        cairo.line_to(self.to.0,self.to.1);
        cairo.stroke();
    }
}