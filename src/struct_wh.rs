struct Body {
    weight: f64,
    height: f64,
}

pub fn main() {
    let ichiro = Body{weight: 70.0, height: 170.0};
    let jiro = Body{weight: 65.0, height: 165.0};
    println!("Ichiro:{:.1}", calc_bmi(&ichiro));
    println!("Jiro:{:.1}", calc_bmi(&jiro));
}

fn calc_bmi(body: &Body) -> f64 {
    body.weight / (body.height / 100.0).powf(2.0)
}