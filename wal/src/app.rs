
pub struct App<Root>
    where Root: Component 
{
    #[allow(dead_code)]
    root: Root,
    vdom: VNode,
    dom: Node,
}

impl<Root> App<Root>
    where Root: Component
{
    pub fn new(root: Root) -> App<Root> {
        let vdom = root.view();
        //App { root }
        todo!()
    } 
}

pub fn start(root: &impl Component) {
   //let app = App {
   //    root
   //};
}

#[cfg(test)]
mod tests {
}