use wal_macros::rsx;

include!("../utils/non_display_struct.rs");

fn main() {
    rsx! { <div class></div> };
    rsx! { <div class=></div> };
    rsx! { <div class "class"></div> };
    rsx! { <div class="class1" class="class2"></div> };
    rsx! { <div class= class="class1"></div> };
    rsx! { <div class={NonDisplayStruct}></div> };

    rsx! { <div wal_class></div> };
    rsx! { <div wal_class=></div> };
    rsx! { <div wal_class ["value"]></div> };
    rsx! { <div wal_class=["class1"] wal_class=["class2"]></div> };
    rsx! { <div wal_class= wal_class=["class1"]></div> };
    rsx! { <div wal_class=[NonDisplayStruct]></div> };
    rsx! { <div wal_class=["class1", "class2", NonDisplayStruct, "class3"]></div> };
}
