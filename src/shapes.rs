use cairo::Context as Cairo;

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
    pub fn new(position : (f64,f64), radius : f64, color : (f64,f64,f64), thickness : f64) -> Circle{
        Circle{
            position,
            radius,
            color,
            thickness
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
impl Line{
    pub fn new(from : (f64,f64), to : (f64,f64), color : (f64,f64,f64), thickness : f64) -> Line{
        Line{
            from,
            to,
            color,
            thickness
        }
    }
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