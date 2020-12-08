extern crate gtk;
extern crate cairo;
extern crate gdk_pixbuf;
extern crate gdk;

use gtk::prelude::*;
use std::sync::{Arc,Mutex};
use std::cell::RefCell;
use fractals::shapes::Shape;
use fractals::*;

fn main() {
    if gtk::init().is_err(){
        println!("Holy shit i couldnt init gtk wtf");
        return;
    }

    let glade_src = include_str!("../app.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window : gtk::Window = builder.get_object("main_window").unwrap();

    build_ui(&builder);
    

    let shapes : Arc<Mutex<Vec<Box<dyn Shape + Send>>>> = Arc::new(Mutex::new(vec!()));
    

    let refresh : gtk::Button = builder.get_object("redraw").unwrap();
    let drawing : gtk::DrawingArea = builder.get_object("drawing").unwrap();
    
    let da : gtk::DrawingArea = builder.get_object("drawing").unwrap();

    let num_x : gtk::Entry = builder.get_object("num_x").unwrap();
    let num_y : gtk::Entry = builder.get_object("num_y").unwrap();
    let type_picker : gtk::ComboBox = builder.get_object("type_picker").unwrap();

    let shapes_down = Arc::downgrade(&shapes);

    let thread_handler = RefCell::new(ThreadHandle::new());

    refresh.connect_clicked(move |_| {
        let size = (drawing.get_allocated_width() as f64, drawing.get_allocated_height() as f64);
        
        let start : f64 = match num_x.get_text().parse() {
            Ok(v) => v,
            Err(_) => 0.0
        };
        let min : f64 = match num_y.get_text().parse() {
            Ok(v) => v,
            Err(_) => 0.0
        };

        let data: FractalInitData = FractalInitData::new(
            shapes_down.clone(),
            size,
            start,
            min
        );

        if thread_handler.borrow().is_thread_running(){
            let thread = thread_handler.replace(ThreadHandle::new());
            thread.kill();
        }

        let f_type = match type_picker.get_active(){
            Some(0) => FractalType::Circles,
            Some(1) => FractalType::Lines,
            Some(2) => FractalType::CirclesWithLines,
            Some(3) => FractalType::Tree,
            Some(4) => FractalType::RandomTree,
            _ =>{
                    println!("NOT IMPLEMENTED");
                    return;
                }
        };

        let new_handle = fractals::generate_fractal(f_type, data);
        let prev_thread = thread_handler.replace(new_handle);

        if prev_thread.is_thread_running(){
            prev_thread.kill();
        }
    });
    // draw the stuff
    da.connect_draw(move |_da, cairo| {
        let shapes = shapes.try_lock();
        match shapes {
            Ok(shapes) => {
                let vec = &*shapes;
                vec.iter().for_each(|c | c.draw(&cairo));

            },
            _ => (),
        };

        gtk::Inhibit(true)
    });
    
    // set the 1 / fps
    glib::timeout_add_local(100, move ||{
        da.queue_draw();
        glib::Continue(true)
    });

    window.connect_destroy(|_| {
        gtk::main_quit();
    });
    window.show_all();
    gtk::main();
}

fn build_ui(builder : &gtk::Builder){
    connect_ticker(&builder, "num_x", 1);
    connect_ticker(&builder, "num_y",1);

    // Set up combo box
    let type_picker : gtk::ComboBox = builder.get_object("type_picker").unwrap();
    let cell = gtk::CellRendererText::new();
    type_picker.pack_start(&cell, false);
    type_picker.add_attribute(&cell, "text", 0);
    // set up the choices
    store_setup(&type_picker);
}

fn store_setup(picker : &gtk::ComboBox){
    let store = gtk::ListStore::new(&[glib::Type::String]);

    store.insert_with_values(None, &[0], &[&"circles"]);
    store.insert_with_values(None, &[0], &[&"lines"]);
    store.insert_with_values(None, &[0], &[&"l+c"]);
    store.insert_with_values(None, &[0], &[&"Tree"]);
    store.insert_with_values(None, &[0], &[&"Random Tree"]);

    picker.set_model(Some(&store));
}

fn connect_ticker(builder : &gtk::Builder, name : &str, step : i32){
    let num : gtk::Entry = builder.get_object(name).expect(&format!("could not find {} in builder",name));
    let add : gtk::Button = builder.get_object(&format!("{}_{}",name,"add")).expect(&format!("could not find  {}_add", name));
    add.connect_clicked(move |_| {
        let number : String = num.get_text().into();
        let number : i32 = match number.parse() {
            Ok(v) => v,
            Err(_) => 0
        };

        let number = number + step;
        num.set_text(&number.to_string());
    });
    let num : gtk::Entry = builder.get_object(name).unwrap(); // no need to expect because it will be cought on the first call
    let sub : gtk::Button = builder.get_object(&format!("{}_{}",name,"sub")).expect(&format!("could not find  {}_sub", name));
    sub.connect_clicked(move |_| {
        let number : String = num.get_text().into();
        let number : i32 = match number.parse() {
            Ok(v) => v,
            Err(_) => 0
        };

        let number = if number - step < 1 { 1 } else { number - step };
        num.set_text(&number.to_string());
    });
    
    let num : gtk::Entry = builder.get_object(name).unwrap();
    num.connect_insert_text(move |s, c, _| {
        if c.chars().into_iter().any(|c| !c.is_numeric() && c != '-'){
            println!("ERROR: {} is not a number",c);
            s.stop_signal_emission("insert_text");
        }
    });
}