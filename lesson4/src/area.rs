use core::f32::consts::PI;

// 圆形结构体
pub struct Circle {
    pub radius: f32,
}
// 正方形结构体
pub struct Square {
    pub len: f32,
}

// 三角形结构体
pub struct Triangle {
    pub bottom: f32,
    pub height: f32,
}

// 计算面积功能约束定义
pub trait CalculatedArea {
    fn area(&self) -> f32;
}
//打印面积
pub fn display_area<T: CalculatedArea>(item: &T) {
    println!(" area is {}", item.area());
}

// 计算圆形面积
impl CalculatedArea for Circle {
    fn area(&self) -> f32 {
        PI * &self.radius * &self.radius
    }
}

// 计算正方形面积
impl CalculatedArea for Square {
    fn area(&self) -> f32 {
        &self.len * &self.len
    }
}
// 计算三角形面积
impl CalculatedArea for Triangle {
    fn area(&self) -> f32 {
        &self.bottom * &self.height / 2.0
    }
}