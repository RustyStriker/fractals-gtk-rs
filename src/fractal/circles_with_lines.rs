use crate::*;

pub fn line_circle_fractals(shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, size : (f64,f64), start : f64, min : f64) -> Option<JoinHandle<()>>{
    // Clear the mutex 
    {
        let shapes = match shapes.upgrade() {
            Some(c) => c,
            _ => return None,
        };
        let mut mutex = shapes.lock().unwrap();
        mutex.clear();

    }
    let h = thread::spawn(move ||{
        lines_circle_fractals_c_helper(shapes.clone(), (size.0 / 2.0,size.1/2.0), start, min, size);
    });
    Some(h)
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
    let circle = Circle::new(
        position,
        length,
        color,
        length / 10.0,
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
    mutex.push(Box::new(circle));
}

fn lines_circle_fractals_l_helper(shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, from : (f64,f64), to : (f64,f64), len : f64, min : f64, size : (f64,f64)){
    let new_len = len * 0.7 ;// 2.0;
    if new_len > min{
        lines_circle_fractals_c_helper(shapes.clone(), to, new_len, min, size);
    }
    let color = (1.0 - (from.1 / size.1), 1.0 - (min / len), 1.0 - (from.0 / size.0));
    let line = Line::new(
        from,
        to,
        color,
        len / 10.0,
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