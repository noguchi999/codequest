trait TreasureBox {
    fn open(&self, key_no: i32) -> bool;
    fn check(&self);
}

struct JewelryBox {
    price: i32,
    key_no: i32
}

impl TreasureBox for JewelryBox {
    fn open(&self, key_no: i32) -> bool {
        self.key_no == key_no
    }

    fn check(&self) {
        println!("宝石箱だった! 金貨: {} 枚入手", self.price);
    }
}

struct TrapBox {
    damage: i32
}

impl TreasureBox for TrapBox {
    fn open(&self, key_no: i32) -> bool {
        return true;
    }

    fn check(&self) {
        println!("罠だった! {} ダメージ", self.damage);
    }
}
