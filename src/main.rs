use macros::demo_proc_macro;

macro_rules! normal_macro {
    ($ normal_macro_input : expr) => {
        let _ = move | a : String | $normal_macro_input;
    } ;
}

macro_rules! call_macro {
    ($name: ident, $($args:expr),*) => {
        $name!($($args,)*)
    }
}

fn main() {
    let a = "some string".to_owned();

    normal_macro!(demo_proc_macro!());

    let _ = a.clone();
}
