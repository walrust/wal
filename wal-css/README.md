# WAL-CSS
Wal-css is a part of the wal project that enables the programmer to use global styling within the application created in wal.

Wal-css provides macro *css_stylesheet* to link the css file to the program and encapsulate it inside Css struct. 
With the Css struct, user can use styles defined within css file locally and not to worry about class name conflicts.
```rust
use wal_css::css:Css;
use wal_css::css_stylesheet;

thread_local! {
    static CSS: Css = css_stylesheet!("path-to-css-file");
}
// ...
CSS.with(|css| {
    rsx! {
        <div class={format!("{} {}", css["class1"], css["class2"])} />
    }
})
```
