trait TreasureBox {
    fn open(&self, key_no: i32) -> bool {
        self.get_key_no() == key_no
    }

    fn check(&self);
    fn get_key_no(&self) -> i32;
}