struct BmiRange {
    min: f64,
    max: f64,
    label: &'static str,
}

pub fn main() {
    let height_cm = input("身長(cm)は？");
    let weight_kg = input("体重(kg)は？");

    let height = height_cm / 100.0;
    let bmi = weight_kg / height.powf(2.0);

    let bmi_list = vec![
        BmiRange { min: 0.0, max: 18.5, label: "低体重" },
        BmiRange { min: 18.5, max: 25.0, label: "普通体重" },
        BmiRange { min: 25.0, max: 30.0, label: "肥満1度" },
    ];
}