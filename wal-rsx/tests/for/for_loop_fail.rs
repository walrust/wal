use wal_rsx::rsx;

include!("../utils/non_display_struct.rs");

fn main() {
    rsx! { for };
    rsx! { for () };
    rsx! { for {} };
    rsx! { for { () } };
    rsx! { for Vec::<()>::new() };
    rsx! { for { Vec::<NonDisplayStruct>::new() } };
    rsx! {
        <div>
            for vec![1, 2, 3]
        </div>
    };
}
