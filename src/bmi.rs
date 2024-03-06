pub fn main() {
    let height_cm = input("身長(cm)は？");
    let weight_kg = input("体重(kg)は？");

    let height = height_cm / 100.0;
    let bmi = weight_kg / height.powf(2.0);
    println!("あなたのBMI値は{:.1}です。", bmi);

    if bmi < 18.5 {println!("痩せ型です")}
    else if bmi < 25.0 {println!("標準です")}
    else if bmi < 30.0 {println!("肥満1度です")}
    else if bmi < 35.0 {println!("肥満2度です")}
    else if bmi < 40.0 {println!("肥満3度です")}
    else {println!("肥満4度です")}
}