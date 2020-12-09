use crate::*;

struct TreeData{
    shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>,
    from : (f64,f64),
    to : (f64,f64),
    size : (f64,f64),
    branches : u32,
}

pub fn random_tree_fractal(data : FractalInitData) -> Option<JoinHandle<()>>{
    // Clear the mutex 
    {
        let shapes = match data.shapes.upgrade() {
            Some(c) => c,
            _ => return None,
        };
        let mut mutex = shapes.lock().unwrap();
        mutex.clear();

    }
    let data = TreeData{
        shapes : data.shapes,
        from : (data.size.0 / 2.0, data.size.1),
        to : (data.size.0 / 2.0, data.size.1 - data.start),
        size : data.size,
        branches : data.end_condition as u32,
    };

    let handle = thread::spawn(move ||{
        random_tree_fractal_helper(data);
    });
    Some(handle)
}

fn random_tree_fractal_helper(data : TreeData){
    {
        let color = (1.0 - (data.from.1 / data.size.1), data.branches as f64 / 255.0, 1.0 - (data.from.0 / data.size.0));
        let line = Line::new(
            data.from, data.to, color,
            2.0,        
        );
        thread::sleep(std::time::Duration::from_millis(1));
        let upgrade = match data.shapes.upgrade() {
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
    if data.branches > 0{
        let len = ((data.to.0 - data.from.0).powi(2) + (data.to.1 - data.from.1).powi(2)).sqrt();
        //let angle = ((to.1 - from.1) / len).asin(); 
        //let angle = if (to.0 - from.0) < 0.0 { angle - 0.5 * 3.14 } else {angle};
        let angle = (data.to.1 - data.from.1).atan2(data.to.0 - data.from.0);
        let len = len * 0.7;

        let mut rand_range = rand::thread_rng();
        let angle_1 = rand::Rng::gen_range(&mut rand_range,0.0, 0.5);
        let angle_2 = rand::Rng::gen_range(&mut rand_range,0.0,0.5);

        let to_1 = in_angle(len, -3.14 * angle_1 + angle, data.to);
        let to_2 = in_angle(len, 3.14 * angle_1 + angle, data.to);
        let to_4 = in_angle(len, -3.14 * angle_2 + angle, data.to);
        let to_5 = in_angle(len, 3.14 * angle_2 + angle, data.to);

        let random = rand::random::<u8>() % 0b1111;
        if random & 0b0001 != 0 {
            random_tree_fractal_helper(TreeData{
                shapes : data.shapes.clone(),
                from : data.to,
                to : to_1,
                branches : data.branches - 1,
                ..data
            });
        }
        if random & 0b0010 != 0 {
            random_tree_fractal_helper(TreeData{
                shapes : data.shapes.clone(),
                from : data.to,
                to : to_4,
                branches : data.branches - 1,
                ..data
            });        }
        if random & 0b0100 != 0 {
            random_tree_fractal_helper(TreeData{
                shapes : data.shapes.clone(),
                from : data.to,
                to : to_2,
                branches : data.branches - 1,
                ..data
            });        }
        if random & 0b1000 != 0 {
            random_tree_fractal_helper(TreeData{
                shapes : data.shapes.clone(),
                from : data.to,
                to : to_5,
                branches : data.branches - 1,
                ..data
            });        }
        
    }
}