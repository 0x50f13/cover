use std::hash::{Hash, Hasher};


#[derive(PartialEq)]
pub struct Point3D{
    pub x: f64,
    pub y: f64,
    pub t: f64,
}

impl Eq for Point3D{}

impl Hash for Point3D{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let _x = self.x as u64;
        let _y = self.y as u64;
        let _t = self.t as u64;
	_x.hash(state);
        _y.hash(state);
        _t.hash(state);
    }
}

impl Point3D{
    pub fn new(_x: f64, _y: f64, _t: f64)-> Point3D{
        Point3D{
            x: _x,
            y: _y,
            t: _t
        }
    }
    pub fn copy(& self) -> Point3D{
        Point3D{ x: self.x, y: self.y, t: self.t}
    }
}

#[derive(PartialEq)]
pub struct Point2D{
    pub x: f64,
    pub y: f64,
}

impl Hash for Point2D{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let _x = self.x as u64;
        let _y = self.y as u64;
        _x.hash(state);
        _y.hash(state);
    }
}

impl Eq for Point2D{}

impl Point2D{
    pub fn new(_x: f64, _y: f64)-> Point2D{
        Point2D{
            x: _x,
            y: _y
        }
    }
    pub fn copy(& self) -> Point2D{
        Point2D{ x: self.x,y: self.y}
    }
}

pub fn rho2d(p1: &Point2D, p2: &Point2D) -> f64{
    let dx = (p1.x-p2.x);
    let dy = (p1.y-p2.y);
    (dx*dx+dy*dy).sqrt()
}

pub fn rho(p1: &Point3D, p2: &Point3D) -> f64{
    let dx = (p1.x-p2.x);
    let dy = (p1.y-p2.y);
    (dx*dx+dy*dy).sqrt()
}
