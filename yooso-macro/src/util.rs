/// Helper method to consume attribute by name and return a vector of all
/// attributes. For example, this consumes all `#[unique]` attributes and
/// returns a vector of their arguments.
pub(crate) fn consume_attributes_by_name(
    attributes: &mut Vec<syn::Attribute>,
    name: &str,
) -> Vec<syn::Attribute> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < attributes.len() {
        if attributes[i].path().is_ident(name) {
            result.push(attributes.remove(i));
        } else {
            i += 1;
        }
    }
    result
}
