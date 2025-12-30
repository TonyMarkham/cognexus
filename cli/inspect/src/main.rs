pub(crate) mod arguments;
pub(crate) mod error;
mod plugin_state;

use crate::arguments::Arguments;
use crate::error::CliError;

use clap::Parser;
use wasmtime::Engine;
use wasmtime::component::Component;
use wasmtime_wasi::p2;

mod types_world {
    wasmtime::component::bindgen!({
        path: "../../wit",
        world: "types-plugin",
    });
}

mod nodes_world {
    wasmtime::component::bindgen!({
        path: "../../wit",
        world: "nodes-plugin",
    });
}

fn main() -> Result<(), CliError> {
    let args = Arguments::parse();

    println!("Loading WASM component: {}", args.wasm_file);

    // Set up engine with component model support
    let mut config = wasmtime::Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;

    // Load the component
    let component = Component::from_file(&engine, &args.wasm_file)?;

    // Create linker and add WASI support
    let mut linker = wasmtime::component::Linker::new(&engine);
    p2::add_to_linker_sync(&mut linker)?;

    // Create store with our state
    let state = plugin_state::PluginState::new();
    let mut store = wasmtime::Store::new(&engine, state);

    match args.kind.as_str() {
        "types" => {
            let plugin = types_world::TypesPlugin::instantiate(&mut store, &component, &linker)?;
            let types = plugin.cognexus_plugin_types().call_list_types(&mut store)?;

            println!("\nFound {} data type(s):", types.len());
            for type_info in types {
                println!("  - {} ({})", type_info.name, type_info.id);
                println!("    Description: {}", type_info.description);
                println!("    Version: {}", type_info.version);
            }
        }
        "nodes" => {
            let plugin = nodes_world::NodesPlugin::instantiate(&mut store, &component, &linker)?;
            let nodes = plugin.cognexus_plugin_nodes().call_list_nodes(&mut store)?;

            println!("\nFound {} node(s):", nodes.len());
            for node_info in nodes {
                println!("  - {} ({})", node_info.name, node_info.id);
                println!("    Description: {}", node_info.description);
                println!("    Version: {}", node_info.version);
                println!("    Input ports: {}", node_info.input_ports.len());
                println!("    Output ports: {}", node_info.output_ports.len());
            }
        }
        _ => {
            eprintln!("Unknown plugin kind: {}. Use 'types' or 'nodes'", args.kind);
            std::process::exit(1);
        }
    }

    Ok(())
}
