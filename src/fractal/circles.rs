use crate::*;

struct CircleData {
    shapes : Weak<Mutex<Vec<Box<dyn Shape + Send>>>>, 
    size : (f64,f64),
    position : (f64,f64),
    length : f64, 
    end_condition : f64
}

pub fn circle_fractals(data : FractalInitData, recv : mpsc::Receiver::<()>) -> Option<JoinHandle<()>>{
    // Clear the mutex
    { 
        let circles = match data.shapes.upgrade() {
            Some(c) => c,
            _ => return None,
        };
        let mut mutex = circles.lock().unwrap();
        mutex.clear();

    }
    let min = data.start / data.end_condition;
    
    let data = CircleData{
        shapes : data.shapes,
        size : data.size,
        end_condition : min,
        length : data.start,
        position : (data.size.0 / 2.0, data.size.1 / 2.0),
    };

    let h = thread::spawn(move || {
        let _ = circle_fractals_helper(data, &recv);
    });
    Some(h)
}

fn circle_fractals_helper(data : CircleData, recv : &mpsc::Receiver::<()>) -> bool{
    match recv.try_recv(){
        Ok(_) => return true,
        Err(mpsc::TryRecvError::Disconnected) => return true,
        _ => ()
    }
    {
        let color = (1.0 - (data.position.1 / data.size.1), 1.0 - (data.end_condition / data.length), 1.0 - (data.position.0 / data.size.0));
        let circle = circle_from_data(&data, color);
        thread::sleep(std::time::Duration::from_millis(10));
        let upgrade = match data.shapes.upgrade() {
            Some(u) => u,
            _ => return true,
        };
        let mut mutex = match upgrade.lock() {
            Ok(lock) => lock,
            _ => return true,
        };
        mutex.push(Box::new(circle));
    }
    let new_len = data.length / 2.0;
    if new_len > data.end_condition{
        let mut quit = false;
        quit |= circle_fractals_helper(CircleData{
            position : (data.position.0 + data.length, data.position.1),
            length : new_len,
            shapes : data.shapes.clone(),
            ..data
        }, recv);
        if quit{
            return true
        }
        quit |= circle_fractals_helper(CircleData{
            position : (data.position.0 - data.length, data.position.1),
            length : new_len,
            shapes : data.shapes.clone(),
            ..data
        }, recv);
        if quit{
            return true
        }
        quit |= circle_fractals_helper(CircleData{
            position : (data.position.0, data.position.1 + data.length),
            length : new_len,
            shapes : data.shapes.clone(),
            ..data
        }, recv);
        if quit{
            return true
        }
        quit |= circle_fractals_helper(CircleData{
            position : (data.position.0, data.position.1 - data.length),
            length : new_len,
            shapes : data.shapes.clone(),
            ..data
        }, recv);
        if quit{
            return true
        }
    }
    false
}

fn circle_from_data(data : &CircleData, color : (f64,f64,f64)) -> Circle{
    Circle::new(data.position, data.length, color, data.length / 10.0)
}
