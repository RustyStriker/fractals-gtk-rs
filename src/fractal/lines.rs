use crate::*;

struct LineData{
    shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, 
    from : (f64,f64),
    to : (f64,f64),
    size : (f64,f64),
    line_amount : u32,
}

pub fn lines_fractal(data : FractalInitData, recv : mpsc::Receiver<()>) -> Option<JoinHandle<()>>{
    let from = (data.size.0 / 2.0, data.size.1 / 2.0);
    
    // Clear the mutex 
    {
        let shapes = match data.shapes.upgrade() {
            Some(c) => c,
            _ => return None,
        };
        let mut mutex = shapes.lock().unwrap();
        mutex.clear();
    }

    let data_left = LineData{
        shapes : data.shapes.clone(),
        from,
        to : (from.0 + data.start, from.1),
        size : data.size,
        line_amount : data.end_condition as u32,
    };
    let data_right = LineData{
        shapes : data.shapes.clone(),
        from,
        to : (from.0 - data.start, from.1),
        size : data.size,
        line_amount : data.end_condition as u32,
    };

    let h = thread::spawn(move ||{
        let quit = lines_fractal_helper(data_left, &recv);
        if quit{
            return;
        }
        let _ = lines_fractal_helper(data_right, &recv);
    });
    Some(h)
}

fn lines_fractal_helper(data : LineData, recv : &mpsc::Receiver<()>) -> bool{
    match recv.try_recv(){
        Ok(()) => return true,
        Err(mpsc::TryRecvError::Disconnected) => return true,
        _ => (),
    }

    let len = (data.to.0 - data.from.0).abs() + (data.to.1 - data.from.1).abs(); // only goes in 1 direction at a time so we cool
    {
        let color = (1.0 - (data.from.1 / data.size.1), 1.0 - (data.line_amount as f64 / len), 1.0 - (data.from.0 / data.size.0));
        let line = Line::new(data.from,data.to,color,len / 10.0);
        thread::sleep(std::time::Duration::from_millis(1));
        let upgrade = match data.shapes.upgrade() {
            Some(u) => u,
            _ => return true,
        };
        let mut mutex = match upgrade.lock() {
            Ok(lock) => lock,
            _ => return true,
        };
        mutex.push(Box::new(line));
    }

    if data.line_amount > 0 {
        let new_len = len * 0.7;
        let dir = (sign(data.to.0 - data.from.0), sign(data.to.1 - data.from.1));
        
        let quit = lines_fractal_helper(LineData{
            shapes : data.shapes.clone(),
            from : data.to,
            to : (data.to.0 + dir.1 * new_len, data.to.1 + dir.0 * new_len),
            line_amount : data.line_amount - 1,
            ..data
        }, recv);
        if quit{
            return true
        }
        let quit = lines_fractal_helper(LineData{
            shapes : data.shapes.clone(),
            from : data.to,
            to : (data.to.0 - dir.1 * new_len, data.to.1 - dir.0 * new_len),
            line_amount : data.line_amount - 1,
            ..data
        }, recv);
        if quit{
            return true
        }
    }
    false
}