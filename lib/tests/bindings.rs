use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};
use interoptopus_backend_csharp::CSharpVisibility;

#[test]
fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    let config = Config {
        visibility_types: CSharpVisibility::ForceInternal,
        ..Default::default()
    };

    Generator::new(
        Config {
            class: "Interop".to_string(),
            dll_name: "libedge_vector_index".to_string(),
            namespace_mappings: NamespaceMappings::new("EdgeVectorIndex"),
            ..config
        },
        edge_vector_index::interop::create_inventory(),
    )
    .write_file("../dotnet/EdgeVectorIndex/Interop.cs")?;

    Ok(())
}

#[test]
fn bindings_c() -> Result<(), Error> {
    use interoptopus_backend_c::{Config, Generator};

    Generator::new(
        Config {
            ifndef: "edge_vector_index".to_string(),
            ..Config::default()
        },
        edge_vector_index::interop::create_inventory(),
    ).write_file("bindings/interop.h")?;

    Ok(())
}