pub fn main() {
    let height_cm = input("身長(cm)は？");
    let weight_kg = input("体重(kg)は？");

    let height = height_cm / 100.0;
    let bmi = weight_kg / height.powf(2.0);
}