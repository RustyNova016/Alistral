use darling::util::PathList;
use darling::FromMeta;
use syn::Field;
use syn::Path;

pub fn field_in_pathlist(field: &Field, list: &PathList) -> bool {
    let identifier = field.ident.as_ref().unwrap();

    let path = match Path::from_string(&identifier.clone().to_string()) {
        Ok(path) => path,
        Err(error) => panic!("Failed to convert field identifier to path: {error:?}"),
    };

    list.contains(&path)
}
