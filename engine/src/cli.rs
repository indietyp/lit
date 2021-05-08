// This file should only be loaded if the cli feature is enabled.

use clap::arg_enum;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::eval::exec::Exec;
use schemars::gen::SchemaSettings;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir, remove_dir_all, write, File};

arg_enum! {
    #[derive(Debug)]
    enum Mode {
        Replace,
        Overwrite
    }
}

#[derive(JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RootSchema {
    node: Node,
    polluted_node: PollutedNode,
    exec: Exec,
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Application {
    /// Generate new JSON-Schema for WASM
    Schema {
        #[structopt(short, long)]
        mode: Mode,

        #[structopt(parse(from_os_str), short, long)]
        output: PathBuf,
    },
}

fn schema(args: Application) -> std::io::Result<()> {
    let (mode, output) = match args {
        Application::Schema { mode, output } => (mode, output),
    };

    if output.is_file() {
        panic!("We need an output, not a directory")
    }

    if let Mode::Replace = mode {
        if output.exists() {
            remove_dir_all(output.clone())?
        }
    }

    if !output.exists() {
        create_dir(output.clone())?;
    }

    let settings = SchemaSettings::default();
    let mut generator = settings.into_generator();

    let root = generator.clone().into_root_schema_for::<RootSchema>();

    let mut path = output.clone();
    path.push("root.json");
    write(path, serde_json::to_string_pretty(&root).unwrap())?;

    Ok(())
}

pub(crate) fn app() {
    let application = Application::from_args();

    match application {
        Application::Schema { .. } => {
            let result = schema(application);
            assert!(
                result.is_ok(),
                "Error happened in Schema generation: {:?}",
                result.err().unwrap()
            )
        }
    };
}
