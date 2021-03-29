pub mod traffic;
pub mod sum;
pub mod area;

fn main() {
    //交通信号灯
    let light = traffic::TrafficLight::Red;
    traffic::display(&light);
    let light = traffic::TrafficLight::Green;
    traffic::display(&light);
    let light = traffic::TrafficLight::Yellow;
    traffic::display(&light);

    //求和
    let items = [10, 20, 35];
    println!("sum is {}", sum::sum(&items).unwrap());

    let items = [2 ^ 32 - 5, 20, 35];
    println!("sum is {}", sum::sum(&items).unwrap());

    // 打印图形面积
    //圆形
    let circle = area::Circle {
        radius: 5.0,
    };
    area::display_area(&circle);

    //正方形
    let square = area::Square {
        len: 10.0
    };
    area::display_area(&square);

    // 三角形
    let triangle = area::Triangle {
        bottom: 3.0,
        height: 5.0,
    };
    area::display_area(&triangle);
}
