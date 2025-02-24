use crate::utils::field_in_pathlist;
use darling::util::PathList;
use darling::FromMeta;
use syn::Fields;
use syn::Path;

pub(crate) fn get_insert_fields_from_idents(fields: &Fields) -> String {
    let mut names = Vec::new();
    for field in fields {
        let identifier = field.ident.as_ref().unwrap();
        names.push(format!("`{}`", identifier));
    }

    format!("({})", names.join(", "))
}

pub(crate) fn get_insert_values_fields_from_idents(
    fields: &Fields,
    ignored_keys: &PathList,
) -> String {
    let mut values = vec![];

    for field in fields {
        if field_in_pathlist(field, ignored_keys) {
            values.push("NULL");
        } else {
            values.push("?");
        }
    }
    for _i in 1..fields.len() {}

    format!("({})", values.join(", "))
}

pub(crate) fn get_on_conflict_fields_from_idents(
    fields: &Fields,
    ignored_keys: &PathList,
) -> String {
    let mut names = Vec::new();
    for field in fields {
        let identifier = field.ident.as_ref().unwrap();

        // Try to convert field identifier to `Path` which is a type provided
        // by `syn`. We do this because `darling`'s PathList type is just a
        // collection of this type with additional methods on it.
        let path = match Path::from_string(&identifier.clone().to_string()) {
            Ok(path) => path,
            Err(error) => panic!("Failed to convert field identifier to path: {error:?}"),
        };

        if ignored_keys.contains(&path) {
            continue;
        }

        names.push(format!("`{}` = excluded.`{}`", identifier, identifier));
    }

    names.join(", ").to_string()
}
