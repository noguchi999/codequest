macro_rules! out_html {
    () => {()};
    ($e:tt) => {println!("{}", $e)};
    ($tag:ident [$($inner:tt)*] $($rest:tt)*) => {{
        print!("<{}>", stringify!($tag));
        out_html!($($inner)*);
        print!("</{}>", stringify!($tag));
        out_html!($($rest)*);
    }};
}

pub fn main() {
    out_html! {
        html [
            head [
                title ["Hello, world!"]
            ]
            body [
                h1 ["Hello, world!"]
                p ["This is a paragraph."]
            ]
        ]
    }
}