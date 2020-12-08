use crate::*;

pub fn random_tree_fractal(shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, size : (f64,f64), start : f64, branches : i32) -> Option<JoinHandle<()>>{
    // Clear the mutex 
    {
        let shapes = match shapes.upgrade() {
            Some(c) => c,
            _ => return None,
        };
        let mut mutex = shapes.lock().unwrap();
        mutex.clear();

    }
    let handle = thread::spawn(move ||{
        random_tree_fractal_helper(shapes.clone(), (size.0 / 2.0, size.1), (size.0 / 2.0, size.1 - start), branches, size);
    });
    Some(handle)
}

fn random_tree_fractal_helper(shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, from : (f64,f64), to : (f64,f64), branches : i32, size : (f64,f64)){
    {
        let color = (1.0 - (from.1 / size.1), branches as f64 / 255.0, 1.0 - (from.0 / size.0));
        let line = Line::new(
            from, to, color,
            2.0,        
        );
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