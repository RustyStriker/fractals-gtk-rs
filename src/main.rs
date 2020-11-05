extern crate gtk;
extern crate cairo;
extern crate gdk_pixbuf;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use cairo::Context as Cairo;


fn main() {
    if gtk::init().is_err(){
        println!("Holy shit i couldnt init gtk wtf");
        return;
    }

    let glade_src = include_str!("../app.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window : gtk::Window = builder.get_object("main_window").unwrap();

    // num_x pretty much all of it
    {
        let num_x : gtk::Entry = builder.get_object("num_x").unwrap();
        let add : gtk::Button = builder.get_object("num_x_add").unwrap();
        add.connect_clicked(move |_| {
            let number : String = num_x.get_text().into();
            let number : i32 = match number.parse() {
                Ok(v) => v,
                Err(_) => 0
            };

            let number = number + 10;
            num_x.set_text(&number.to_string());
        });
        let num_x : gtk::Entry = builder.get_object("num_x").unwrap();
        let sub : gtk::Button = builder.get_object("num_x_sub").unwrap();
        sub.connect_clicked(move |_| {
            let number : String = num_x.get_text().into();
            let number : i32 = match number.parse() {
                Ok(v) => v,
                Err(_) => 0
            };

            let number = if number - 10 < 1 { 1 } else { number - 10 };
            num_x.set_text(&number.to_string());
        });
    }
    let num_x : gtk::Entry = builder.get_object("num_x").unwrap();
    // let drawing : gtk::DrawingArea = builder.get_object("drawing").unwrap();

    // num_x.connect_changed(move |_| {
    //     drawing.queue_draw();
    // });
    // Do not accept any non numeric char
    num_x.connect_insert_text(|s, c, _| {
        if c.chars().into_iter().any(|c| !c.is_numeric() && c != '-'){
            println!("num_x: {} is not a number",c);
            s.stop_signal_emission("insert_text");
        }
    });
    // num_y - basically a copy of num_x
    {
        let num_y : gtk::Entry = builder.get_object("num_y").unwrap();
        let add : gtk::Button = builder.get_object("num_y_add").unwrap();
        add.connect_clicked(move |_| {
            let number : String = num_y.get_text().into();
            let number : i32 = match number.parse() {
                Ok(v) => v,
                Err(_) => 0
            };

            let number = number + 10;
            num_y.set_text(&number.to_string());
        });
        let num_y : gtk::Entry = builder.get_object("num_y").unwrap();
        let sub : gtk::Button = builder.get_object("num_y_sub").unwrap();
        sub.connect_clicked(move |_| {
            let number : String = num_y.get_text().into();
            let number : i32 = match number.parse() {
                Ok(v) => v,
                Err(_) => 0
            };

            let number = if number - 10 < 1 { 1 } else { number - 10 };

            num_y.set_text(&number.to_string());
        });
    }
    let num_y : gtk::Entry = builder.get_object("num_y").unwrap();
    // let drawing : gtk::DrawingArea = builder.get_object("drawing").unwrap();
    // num_y.connect_changed(move |_| {
    //     drawing.queue_draw();
    // });
    // Do not accept any non numeric char
    num_y.connect_insert_text(|s, c, _| {
        if c.chars().into_iter().any(|c| !c.is_numeric() && c != '-'){
            println!("num_y: {} is not a number",c);
            s.stop_signal_emission("insert_text");
        }
    });

    // Color picker stuff
    // let color = connect_color_picker(&builder);
    
    let flipper = Rc::new(RefCell::new(false));
    let flipper_copy = Rc::downgrade(&flipper);

    
    let refresh : gtk::Button = builder.get_object("redraw").unwrap();
    let drawing : gtk::DrawingArea = builder.get_object("drawing").unwrap();
    
    let da : gtk::DrawingArea = builder.get_object("drawing").unwrap();
    let drawing_size = da.get_allocation();
    //let pixbuf = gdk_pixbuf::Pixbuf::new(gdk_pixbuf::Colorspace::Rgb, false, 8, drawing_size.width, drawing_size.height);
    //let pixbuf = Rc::new(pixbuf.unwrap());
    //let pix_copy = Rc::downgrade(&pixbuf);
    let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, drawing_size.width, drawing_size.height).unwrap();
    let surface = Rc::new(surface);
    let sur_copy = Rc::downgrade(&surface);


    refresh.connect_clicked(move |_| {
        let flippy = flipper_copy.upgrade().unwrap();
        let mut flippy = flippy.borrow_mut();
        *flippy = true;

        
        let cairo = cairo::Context::new(&surface);

        let start : f64 = match num_x.get_text().parse() {
            Ok(v) => v,
            Err(_) => 0.0
        };
        let min : f64 = match num_y.get_text().parse() {
            Ok(v) => v,
            Err(_) => 0.0
        };

        fractals::circle_fractals(&cairo, (surface.get_width() as f64, surface.get_height() as f64),start, start / min);
        println!("render");

        drawing.queue_draw();
    });
    // draw the stuff
    da.connect_draw(move |_da, cairo| {
        //let color = color.borrow();
        //cairo.set_source_rgb(color.0, color.1, color.2);
        
        // let pos_x : f64 = match num_x.get_text().parse() {
        //     Ok(v) => v,
        //     Err(_) => 0.0
        // };
        // let pos_y : f64 = match num_y.get_text().parse() {
        //     Ok(v) => v,
        //     Err(_) => 0.0
        // };
        // let mut flipper = flipper.borrow_mut();
        // if *flipper{
        //     //fractals::circle_fractals(da,cairo,pos_x,pos_x / pos_y);
        //     *flipper = false;
        // }
        // else{
            
        // }
        let surface = sur_copy.upgrade().unwrap();
        cairo.set_source_surface(&surface, 0.0, 0.0);
        cairo.paint();
        println!("Drawing");


        gtk::Inhibit(true)
    });
    

    window.connect_destroy(|_| {
        gtk::main_quit();
    });
    window.show_all();
    gtk::main();
}



fn connect_color_picker(builder : &gtk::Builder) -> Rc<RefCell<(f64,f64,f64)>>{
    let color_dia : gtk::Button = builder.get_object("color_dialog").unwrap();
    let color = Rc::new(RefCell::new((0.0,0.0,0.0)));
    let color_copy = Rc::downgrade(&color);
    let color_picker : gtk::ColorChooserDialog = builder.get_object("color_picker").unwrap();
    color_picker.connect_hide(move |s|{
        let color = color_copy.upgrade().unwrap();
        let mut color = color.borrow_mut();
        let r = s.get_rgba();
        (*color) = (r.red, r.green, r.blue);
    });
    color_dia.connect_clicked(move |_| {
        let _ = color_picker.run();
        color_picker.hide();
    });
    color
}