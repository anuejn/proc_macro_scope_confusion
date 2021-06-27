use macros::demo_proc_macro;

macro_rules! normal_macro {
    ($($input:tt)*) => {
        let _ = move | a : String | $($input)*;
    } ;
}

fn main() {
    let a = "some string".to_owned();

    normal_macro!(demo_proc_macro!());

    let _ = a.clone();
}
