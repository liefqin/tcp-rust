
pub trait TrafficTime{
    fn time(&self) -> u32;
}

pub enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficTime for TrafficLight {
    fn time(&self) -> u32{
        match &self{
            TrafficLight::Red=>20,
            TrafficLight::Green=>30,
            TrafficLight::Yellow=>3,
        }
    }
}

pub fn display(traffic_light: &impl TrafficTime){
    println!("The light time is {}", traffic_light.time());
}
