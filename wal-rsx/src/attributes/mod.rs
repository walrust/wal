use std::collections::HashMap;

pub(crate) mod event_attribute;
pub(crate) mod normal_attribute;
pub(crate) mod props_attribute;
pub(crate) mod wal_class_attribute;

pub(crate) const KEY_ATTR: &str = "key";

pub(crate) trait Attribute {
    type AttributeValue: Clone;

    fn ident(&self) -> &proc_macro2::Ident;

    fn value(&self) -> &Self::AttributeValue;
}

pub(crate) fn process_specialized_attribute<Attr: Attribute>(
    attribute: &mut Option<Attr>,
    incoming_attribute: Attr,
) -> syn::Result<()> {
    if attribute.is_some() {
        let incoming_attribute_ident = incoming_attribute.ident();
        return Err(duplicate_attribute_error(incoming_attribute_ident));
    }
    *attribute = Some(incoming_attribute);
    Ok(())
}

pub(crate) fn process_unspecialized_attribute<Attr: Attribute>(
    attributes: &mut HashMap<proc_macro2::Ident, Attr::AttributeValue>,
    incoming_attribute: &Attr,
) -> syn::Result<()> {
    let incoming_attribute_ident = incoming_attribute.ident();
    if attributes
        .insert(
            incoming_attribute_ident.clone(),
            incoming_attribute.value().clone(),
        )
        .is_some()
    {
        Err(duplicate_attribute_error(incoming_attribute_ident))
    } else {
        Ok(())
    }
}

fn duplicate_attribute_error(ident: &proc_macro2::Ident) -> syn::Error {
    syn::Error::new(ident.span(), format!("Duplicate attribute `{ident}`"))
}
