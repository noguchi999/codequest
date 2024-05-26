macro_rules! bmi_select {
    ($bmi:expr; $($lable:expr => $range:expr); *) => {{
        let mut result = "error";
        $(
            if $range.start <= $bmi {
                result = $lable;
            }
        ) +
        result
    }};
}