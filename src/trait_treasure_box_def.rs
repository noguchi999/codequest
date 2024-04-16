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
