macro_rules! easy_for {
    (for !i:indent = &from:tt to $to:tt $block:block) => {{
        for $i in $from..=$to {
            $block
        }
    }};

    (for !i:indent = &from:tt to $to:tt step $step:tt $block:block) => {{
        let mut $i = $from;
        loop {
            if $i < $to {break}
            $block
            $i += $step;
        }
    }};
}

pub fn main() {
    let mut total = 0;
    easy_for! {
        for i = 1 to 10 {
            total += i;
        }
    }
    println!("Total: {}", total);

    easy_for! {
        for i = 1 to 10 step 3 {
            println!("{}", i);
        }
    }
}