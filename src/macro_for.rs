macro_rules! easy_for {
    (for !i:indent = &from:tt to $to:tt $block:block) => {{
        for $i in $from..=$to {
            $block
        }
    }};

}