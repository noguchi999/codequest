trait TreasureBox {
    fn open(&self, key_no: i32) -> bool {
        self.get_key_no() == key_no
    }

    fn check(&self);
    fn get_key_no(&self) -> i32;
}

struct JewelBox {
    price: i32,
    key_no: i32,
}
impl TreasureBox for JewelBox {
    fn check(&self) {
        println!("宝石箱だった! 金貨{}枚を入手。", self.price);
    }

    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

struct EmptyBox {
    key_no: i32,
}
impl TreasureBox for EmptyBox {
    fn check(&self) {
        println!("空箱だった。");
    }

    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if !tbox.open(key_no) {
        println!("鍵が合わない！！");
        return;
    }
    tbox.check();
}

pub fn main() {
    let box1 = JewelBox { price: 30, key_no: 1 };
    let box2 = EmptyBox { key_no: 1 };
    let box3 = JewelBox { price: 50, key_no: 2 };

    open_box(&box1, 1);
    open_box(&box2, 1);
    open_box(&box3, 1);
}