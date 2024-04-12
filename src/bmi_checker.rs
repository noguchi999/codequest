struct BmiRange {
    min: f64,
    max: f64,
    label: String,
}

impl BmiRange {
    fn new(min: f64, max: f64, label: &str) -> Self {
        Self { min, max, label: label.to_string() }
    }

    fn test(&self, v: f64) -> bool {
        self.min <= v && v < self.max
    }
}

struct Body {
    height: f64,
    weight: f64,
    name: String
}

impl Body {
    fn new(height: f64, weight: f64, name: &str) -> Self {
        Self { height, weight, name: name.to_string() }
    }

    fn calc_bmi(&self) -> f64 {
        self.weight / (self.height / 100.0).powf(2.0)
    }

    fn print_result(&self) {
        let bmi = self.calc_bmi();
        let bmi_list = [
            BmiRange::new(0.0, 18.5, "低体重"),
            BmiRange::new(18.5, 25.0, "普通体重"),
            BmiRange::new(25.0, 30.0, "肥満1度"),
            BmiRange::new(30.0, 35.0, "肥満2度"),
            BmiRange::new(35.0, 40.0, "肥満3度"),
            BmiRange::new(40.0, 99.9, "肥満4度")
        ];

        let mut result = String::from("不明");
        for range in bmi_list {
            if range.test(bmi) {
                result = range.label.clone();
                break;
            }
        }
        println!("{}さんのBMIは{:.1}で、{}です。", self.name, bmi, result)
    }
}

pub fn main() {
    let body = Body::new(170.0, 65.0, "田中");
    body.print_result();
    let body = Body::new(160.0, 70.0, "鈴木");
    body.print_result();
    let body = Body::new(180.0, 100.0, "佐藤");
    body.print_result();
}
