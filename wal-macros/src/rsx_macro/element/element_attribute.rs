use syn::parse::{Parse, ParseStream};

use crate::rsx_macro::attributes::{
    event_attribute::{EventAttribute, IsEvent},
    normal_attribute::NormalAttribute,
    wal_class_attribute::WalClassAttribute, KEY_ATTR,
};

const CLASS_ATTR: &str = "class";
const WAL_CLASS_ATTR: &str = "wal_class";

pub enum ElementAttribute {
    Normal(NormalAttribute),
    Event(EventAttribute),
    Key(NormalAttribute),
    Class(NormalAttribute),
    WalClass(WalClassAttribute),
}

impl Parse for ElementAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let forked_input = input.fork();
        let ident: proc_macro2::Ident = forked_input.parse()?;

        if ident == KEY_ATTR {
            Ok(ElementAttribute::Key(input.parse()?))
        } else if ident == CLASS_ATTR {
            Ok(ElementAttribute::Class(input.parse()?))
        } else if ident == WAL_CLASS_ATTR {
            Ok(ElementAttribute::WalClass(input.parse()?))
        } else if ident.is_event() {
            Ok(ElementAttribute::Event(input.parse()?))
        } else {
            Ok(ElementAttribute::Normal(input.parse()?))
        }
    }
}

impl ElementAttribute {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(syn::Ident)
    }
}
