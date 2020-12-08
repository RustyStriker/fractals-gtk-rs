extern crate gtk;
extern crate cairo;

use std::sync::{Weak,Mutex,mpsc};
use std::{thread,thread::JoinHandle};
use rand;
pub mod shapes;
use shapes::*;

mod fractal;

pub struct FractalInitData{
    shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, 
    size : (f64,f64), 
    start : f64, 
    end_condition : f64
}

impl FractalInitData{
    pub fn new(shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, size : (f64,f64), start : f64, end_condition : f64) -> FractalInitData{
        FractalInitData{
            shapes,
            size,
            start,
            end_condition
        }
    }
}

pub enum FractalType{
    Circles,
    Lines,
    CirclesWithLines,
    Tree,
    RandomTree,
}

pub fn generate_fractal(f_type : FractalType, data : FractalInitData) -> ThreadHandle{
    let (send,recv) = mpsc::channel::<()>();

    let handle = match f_type{
        FractalType::Circles => fractal::circles::circle_fractals(data, recv),
        FractalType::Lines => fractal::lines::lines_fractal(data, recv),
        FractalType::Tree => fractal::tree::tree_fractal(data),
        _ => None,
    };
    ThreadHandle{
        thread : handle,
        channel : Some(send),
    }
}

pub struct ThreadHandle {
    thread : Option<JoinHandle<()>>,
    channel : Option<mpsc::Sender<()>>
}
impl ThreadHandle{
    pub fn new() -> ThreadHandle{
        ThreadHandle{
            thread : None,
            channel : None,
        }
    }

    pub fn kill(self){
        let thread = match self.thread{
            Some(t) => t,
            None => return
        };
        
        let channel = self.channel;
        let channel = match channel {
            Some(c) => c,
            None => panic!("found a thread, but no channel was created for it")
        };
        let _ = channel.send(());
        let _ = thread.join();
    }

    pub fn is_thread_running(&self) -> bool{
        match self.thread{
            Some(_) => true,
            None => false,
        }
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

fn in_angle(len : f64, angle : f64, from : (f64,f64)) -> (f64,f64){
    (len * angle.cos() + from.0, len * angle.sin() + from.1)
}

