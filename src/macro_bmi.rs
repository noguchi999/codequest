macro_rules! bmi_select {
    ($bmi:expr; $($lable:expr => $range:expr); *) => {{
        let mut result = "error";
        $(
            if $range.start <= $bmi {
                result = $lable;
            }
        ) +
        result
    }};
}

pub fn main() {
    let h: f32 = 158.0;
    let w: f32 = 50.0;
    let bmi = w / (h / 100.0).powf(2.0);
    let lavel = bmi_select![
        bmi;
        "Underweight" => 0.0..18.5;
        "Normal" => 18.5..25.0;
        "Overweight" => 25.0..30.0;
        "Obese" => 30.0..std::f32::INFINITY;
    ];
}