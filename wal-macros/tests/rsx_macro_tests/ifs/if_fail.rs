use wal_macros::rsx;

fn main() {
    rsx! { if };
    rsx! { if true };
    rsx! { if {} };
    rsx! { if 1 {} };
    let x = 1;
    rsx! { if x {} };
    rsx! { if true {} else };
    rsx! { if true {} else if };
    rsx! { if true {} else if true };
    rsx! { if true {} else if {} };
    rsx! { if true {} else if true {} else };
    rsx! { if true { {} } };
    rsx! { if true { () } };
    rsx! { if true { invalid_reference } };
}
