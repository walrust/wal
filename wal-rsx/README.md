# Wal-rsx

Wal-rsx is a part of wal project that enables the programmer to define user interface. The user interface could be defined without using this crate, but it is not a recommended approach.

Wal-rsx provide only a single rsx macro. With the usage of this macro you could write HTML like syntax to define user interface:

```rust
rsx! {
    <div id="identifier" class="class">
        { 1 + 1 }
    </div>
    <button onclick={|event: MouseEvent| {
        println!("I love wal rsx!");
    }}> "I love wal rsx!" </button>
}
```
