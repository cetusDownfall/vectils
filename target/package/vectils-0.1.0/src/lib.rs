//! This crate provides vectors and operations 
//! on vectors which are probably worse than 
//! most other implementations.
use std::ops::{Add, Sub, Mul, Div};
/// This trait allows the creation of 2 dimensional vectors over various types.
pub trait Vector2d<T>
where T: PartialEq + PartialOrd 
       + Copy + Clone
       + Add<Output=T> + Sub<Output=T> 
       + Mul<Output=T> + Div<Output=T>
{
    /// X value of first point.
    fn xi(&self) -> T; 
    /// Y value of first point.
    fn yi(&self) -> T;
    /// X value of second point.
    fn xf(&self) -> T; 
    /// Y value of second point.
    fn yf(&self) -> T;
    /// Associated helper function which checks for whether or not a number falls within a range
    /// (inclusive).
    fn in_range(mut a: T, mut b: T, c: T) -> bool {
        if a > b { let tmp = a; a = b; b = tmp; }
        !(a > c || b < c)
    }
    /// Returns width of the vector. Signed.
    fn w(&self) -> T { self.xf() - self.xi() }
    /// Returns height of the vector. Signed.
    fn h(&self) -> T { self.yf() - self.yi() }
    /// Returns the slope of the vector.
    fn delta(&self) -> T { self.w() / self.h() }
    /// Returns whether or not this vector's domain includes the x value provided.
    fn in_dom(&self, pt_x: T) -> bool { <Self as Vector2d<T>>::in_range(self.xi(), self.xf(), pt_x) }
    /// Returns the y value of the line which is made by extending the ends of this vector.
    fn linear(&self, pt_x: T) -> T { self.delta() * pt_x + self.yi() }
    /// Returns the point at which two vectors cross, or None if they do not.
    fn cross<O: Vector2d<T>>(&self, b: &O) -> Option<[T;2]> {
        if self.delta() != b.delta() {
            let insct = ((self.yi() - b.yi()) * self.w() * b.w()) /
                         (b.h() * self.w() - self.h() * b.w());
            if self.in_dom(insct) && b.in_dom(insct) { Some([insct, self.linear(insct)]) } 
            else { None }
        } else { None }
    }
}
impl<T> Vector2d<T> for (T,T,T,T)
    where T: PartialEq + PartialOrd 
           + Copy + Clone
           + Add<Output=T> + Sub<Output=T> 
           + Mul<Output=T> + Div<Output=T>
{
    fn xi(&self) -> T { self.0 }
    fn yi(&self) -> T { self.1 }
    fn xf(&self) -> T { self.2 }
    fn yf(&self) -> T { self.3 }
}
impl<T> Vector2d<T> for [T;4] 
    where T: PartialEq + PartialOrd 
           + Copy + Clone
           + Add<Output=T> + Sub<Output=T> 
           + Mul<Output=T> + Div<Output=T>
{
    fn xi(&self) -> T { self[0] }
    fn yi(&self) -> T { self[1] }
    fn xf(&self) -> T { self[2] }
    fn yf(&self) -> T { self[3] }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cross() {
        use Vector2d;
        let v1 = [0.0f64, 0., 5., 5.];
        let v2 = [5.0f64, 0., 0., 5.];
        let p1 = v1.cross(&v2);
        let p2 = v2.cross(&v1);
        assert_eq!(p1,p2);
    }
}
