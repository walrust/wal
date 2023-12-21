//! This crate provides the `rsx!` procedural macro for writing JSX-like syntax in Rust.
//! This macro is used to define views and behaviors of web application.
use proc_macro::TokenStream;
use quote::ToTokens;
use root::Root;
use syn::parse_macro_input;

mod attributes;
mod component;
mod element;
mod expression_block;
mod r#for;
mod forest;
mod fragment;
mod r#if;
mod link;
mod literal;
mod root;
mod tree;

/// The `rsx!` procedural macro allows you to write JSX-like syntax in Rust.
/// This macro is used to define views and behaviors of web applcation.
///
/// A simple usage could look like this:
///
/// ```
/// use wal_rsx::rsx;
///
/// rsx! {
///     <div class="container">
///         <h1>{ "Hello, world!" }</h1>
///     </div>
/// };
/// ```
/// But this macro supports much more than that simple example
///
/// ## Literals
/// The supported literals are:
/// - A UTF-8 string literal: `"foo"`
/// - A byte literal: `b'a'`
/// - A character literal: `'a'`
/// - An integer literal: `1`, `1i32`, `1u32`, ...
/// - A floating point literal: `1.0`, `1.0f32`, `1.0e1`, ... .
/// The only restriction here is that it must be finite. So it can not be infinity or NaN.
/// - A boolean literal: `true` or `false`
///
/// A byte string literal: `b"foo"` is not supported.
/// The reason behind this is that it does not implement [Display](::std::fmt::Display) trait, meaning it can not be displayed.
///
/// The example usage of literals:
///
/// ```
/// use wal_rsx::rsx;
///
/// rsx! { 1 };
/// ```
///
/// ## Expressions
/// Macro supports every expression that can be used in Rust.
/// The only restriction is that the result of the expression must implement [Display](::std::fmt::Display) trait, so it can be displayed.
///
/// The example usage of expressions:
///
/// ```
/// use wal_rsx::rsx;
///
/// rsx! { <div> { 1 + 1 } </div> };
/// ```
///
/// The important note is that expressions must be wrapped in curly braces `{}`.
/// The only exception is when the expression is the only element in a macro:
///
/// ```
/// use wal_rsx::rsx;
///
/// rsx! { 1 + 1 };
/// ```
///
/// ## Elements
/// Macro supports elements from HTML that could go inside a `body` element.
/// The complete list could be found [here](https://developer.mozilla.org/en-US/docs/Web/HTML/Element).
/// The only exceptions are: SVG, MathML and scripting elements.
///
/// Elements can have childrens, which could be anything that is supported by this macro except:
/// - expression not wrapped in curly braces `{}`
/// - for expression not wrapped in curly braces `{}`
///
/// Void elements can not have childrens and they must be self closing.
/// In the other hand, normal elements can have childrens, but they can also be self closing.
///
/// The example usage of elements:
///
/// ```
/// use wal_rsx::rsx;
///
/// rsx! {
///     <div>
///         <h1> { "Hello, world!" } </h1>
///     </div>
///     <div/>
/// };
/// ```
///
/// #### Attributes
/// Elements supports many kinds of attributes.
///
/// ###### Normal attributes
/// Normal attributes are attributes to which, in HTML, a string is assigned.
/// To normal attributes belong: `class`, `id`, `style`, ...
/// Normal attributes can be assigned with a literal or an expression wrapped in curly braces `{}`.
/// The only restriction is that the result of the expression must implement [Display](::std::fmt::Display) trait, so it supports [to_string()](::std::string::ToString::to_string) function.
///
/// The example usage of normal attributes:
///
/// ```
/// use wal_rsx::rsx;
///
/// rsx! { <div class="container" id={"main"}  /> };
/// ```
///
/// ###### Event attributes
/// Event attributes are attributes to which, in HTML, a function reprsesnting a behavior is assigned.
/// To event attributes belong: `onclick`, `oninput`, `onchange`, ...
/// Event attributes can be assigned with an expression wrapped in curly braces `{}`.
/// The only restriction is that the result of the expression must return a [`Callback<IN>`](../wal/component/callback/struct.Callback.html) where `IN` is a type of event.
/// For example when assigning action to be called while the element is clicked,
/// [`Callback<MouseEvent>`](../wal/component/callback/struct.Callback.html)
/// (see [MouseEvent](../wal/events/struct.MouseEvent.html)) must be returned.
///
/// The example usage of event attributes:
///
/// ```
/// use wal_rsx::rsx;
/// use wal_core::component::callback::Callback;
/// use wal_core::events::MouseEvent;
///
/// rsx! {
///     <button onclick={Callback::new(|event: MouseEvent| {
///         // define any action here that should be executed when the button is clicked
///     })} />
/// };
/// ```
///
/// ###### Wal class attribute
/// Defining a `class` attribute could be accomplished by using a normal attribute `class`.
/// The problem with that is that it can become very verbose and bloated while defining many classes from for example variables.
/// It might not seem problematic at first, cause having many classes in a variables seems to be not a common case,
/// but it is a common case while using wal-css which is a recommended way of styling in wal.
/// It might leed to code like this:
///
/// ```no_run
/// use wal_rsx::rsx;
/// use wal_css::css::Css;
/// use wal_css::css_stylesheet;
///
/// thread_local! {
///     static CSS: Css = css_stylesheet!("path-to-css-file.css");
/// }
/// // ...
/// CSS.with(|css| {
///     rsx! {
///         <div class={format!("{} {}", css["class1"], css["class2"])} />
///     };
/// });
/// ```
///
/// The solution to this problem is to use a `wal_class` attribute.
/// Wal class attribute can be assigned with an array expression.
/// Elements of this array must implement [Display](::std::fmt::Display) trait, so that they support [to_string()](::std::string::ToString::to_string) function.
/// The usage of this attribute simplifies the code above to this:
///
/// ```no_run
/// use wal_rsx::rsx;
/// use wal_css::css::Css;
/// use wal_css::css_stylesheet;
///
/// thread_local! {
///     static CSS: Css = css_stylesheet!("path-to-css-file.css");
/// }
/// // ...
/// CSS.with(|css| {
///     rsx! {
///         <div wal_class=[css["class1"], css["class2"]] />
///     };
/// });
/// ```
///
/// Both attributes `class` and `wal_class` can be used at the same time.
/// Using both of them will result in merging classes from both attributes.
///
/// ###### Key attribute
/// Elements support `key` attribute. More about this attribute could be found [here](#key-attribute-1).
///
/// ## Key attribute
/// Key attribute is used to identify a node. Two nodes with the same key are considered to be the same.
///
/// Key attribute can be assigned with a literal or an expression wrapped in curly braces `{}`.
/// The only restriction is that the result of the expression must implement `Display` trait, so it supports `to_string` function.
/// Using key attribute can lead to performance improvements because:
/// - there is no need running algorithm for comparing two nodes if their `key` attributes are equal
/// - there is no need to rerender a node if it has the same `key` as the previous one
///
/// The example usage of `key` attribute:
///
/// ```
/// use wal_rsx::rsx;
///
/// rsx! { <div key="key1" /> };
/// ```
///
/// ## Fragments
/// Fragments are used to group multiple nodes together. They are not rendered to the DOM.
/// Their main purpose is to return a single root node.
/// Their usage is completely optional and a matter of a personal choice as they will be added automatically, if multiple root
/// nodes are to be returned.
///
/// Fragments can have childrens, which could be anything that is supported by this macro except:
/// - expression not wrapped in curly braces `{}`
/// - for expression not wrapped in curly braces `{}`
///
/// Fragments support `key` attribute. More about this attribute could be found [here](#key-attribute-1).
///
/// The example usage of fragments:
///
/// ```
/// use wal_rsx::rsx;
///
/// rsx! {
///     <>
///         <div/>
///         <div/>
///     </>
/// };
/// ```
///
/// The example above is equivalent to:
///
/// ```
/// use wal_rsx::rsx;
///
/// rsx! {
///     <div/>
///     <div/>
/// };
/// ```
///
/// ## For loops
/// Macro supports for loops, which are used to generate view for every element of a collection.
/// The only restriction is that the transformated item must be convertible to [VNode](../wal/virtual_dom/vnode/enum.VNode.html).
/// So it must implement one of the following traits:
/// - `Display` so that it can be displayed
/// - `Into<VNode>` so that it can be converted to [VNode](../wal/virtual_dom/vnode/enum.VNode.html).
///
/// The example usage of for loops:
///
/// - simple example displaying every element of a vector using [to_string()](::std::string::ToString::to_string) function from [Display](::std::fmt::Display) trait
/// ```
/// use wal_rsx::rsx;
///
/// rsx! {
///     <div>
///         for { vec!["a", "b", "c"] }
///     </div>
/// };
/// ```
///
/// - more complex example displaying every element of a vector after some transformation
/// ```
/// use wal_rsx::rsx;
///
/// rsx! {
///     <div>
///         for {
///             vec!["a", "b", "c"]
///                 .iter()
///                 .map(|x| rsx! { <div> { x } </div> })
///         }
///     </div>
/// };
/// ```
///
/// The important note is that collection must be wrapped in curly braces `{}`.
/// The only exception is when the for loop is the only element in a macro:
///
/// ```
/// use wal_rsx::rsx;
///
/// rsx! { for [0, 1, 2] };
/// ```
///
/// ## If expressions
/// Macro supports if expressions, which are used to conditionally render a node.
/// `if`, `else if`, `else`, `if let` and `if let else` are supported.
/// Inside a `if` body can be anything that is supported by this macro.
///
/// The example usage of if expressions:
///
/// ```
/// use wal_rsx::rsx;
///
/// rsx! {
///     if true {
///         <div/>
///     } else if false {
///         <div/>
///     } else if let Some(x) = Some(5) {
///         x
///     } else {
///         <div/>
///     }
/// };
/// ```
///
/// ## Links
/// Macro supports links, which are used to perform routing.
///
/// Links support `to` attribute, which is used to define a path to which the link should route.
/// `to` attribute can be assigned with a literal or an expression wrapped in curly braces `{}`.
/// The only restriction is that the result of the expression must implement [Display](::std::fmt::Display) trait, so it supports [to_string()](::std::string::ToString::to_string) function.
/// Links support `key` attribute. More about this attribute could be found [here](#key-attribute-1).
///
/// Links can have childrens, which could be anything that is supported by this macro except:
/// - expression not wrapped in curly braces `{}`
/// - for expression not wrapped in curly braces `{}`
///
/// Clicking on any children of Link will result in routing to the path defined in `to` attribute.
///
/// The example usage of links:
///
/// ```
/// use wal_rsx::rsx;
///
/// rsx! {
///    <Link to="/path"> "Click me to change route" </Link>
/// };
/// ```
///
/// ## Custom components
/// Macro supports custom components, which are used to define reusable, custom views and behaviors.
/// Custom components can not have children.
/// Custom components support `props` attribute. This attribute is used to pass properties to the component.
/// `props` attribute can be assigned with a literal or an expression wrapped in curly braces `{}` or a struct expression.
/// The only restriction is that the value must be convertable to a
/// [Properties](../wal/component/trait.Component.html#associatedtype.Properties) type of the
/// [Component](../wal/component/trait.Component.html).
/// Custom components support `key` attribute. More about this attribute could be found [here](#key-attribute-1).
///
/// The example usage of custom components:
///
/// ```
/// use wal_rsx::rsx;
/// use wal_core::component::Component;
/// use wal_core::component::behavior::Behavior;
/// use wal_core::virtual_dom::VNode;
///
/// struct MyComponent {
///     x: i32,
/// }
///
/// #[derive(Hash)]
/// struct MyComponentProps {
///     x: i32,
/// }
///
/// impl Component for MyComponent {
///     type Properties = MyComponentProps;
///
///     type Message = ();
///
///     fn new(props: Self::Properties) -> Self {
///         Self { x: props.x }
///     }
///
///     fn view(&self, _behavior: &mut impl Behavior<Self>) -> VNode {
///         rsx! { <div> { self.x } </div> }
///     }
///     
///     fn update(&mut self, _message: Self::Message) -> bool {
///         false
///     }
/// }
///
/// // ...
///
/// rsx! {
///     <MyComponent props= MyComponentProps { x: 1 } />
/// };
/// ```
///
/// ## Empty
/// Macro supports empty input. It is used to return an empty view.
///
/// The example usage of empty input:
///
/// ```
/// use wal_rsx::rsx;
///
/// rsx! {};
/// ```
#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as Root);
    TokenStream::from(root.into_token_stream())
}
