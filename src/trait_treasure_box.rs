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

fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if tbox.open(key_no) {
        tbox.check();
    } else {
        println!("鍵が合わない...");
    }
}

pub fn main() {
    let box_1 = JewelryBox { price: 100, key_no: 1 };
    let box_2 = TrapBox { damage: 50 };
    let box_3 = JewelryBox { price: 200, key_no: 2 };

    let my_key = 2;
    open_box(&box_1, my_key);
    open_box(&box_2, my_key);
    open_box(&box_3, my_key);
}