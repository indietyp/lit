// This file should only be loaded if the cli feature is enabled.

use clap::arg_enum;
use std::path::PathBuf;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    enum Mode {
        Overwrite
    }
}

#[derive(Debug, Clone, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Application {
    Schema {
        #[structopt(short, long)]
        mode: Mode,

        #[structopt(parse(from_os_str), short, long)]
        output: PathBuf,
    },
}

fn typescript(args: Application) {
    let (mode, output) = match args {
        Application::Schema { mode, output } => (mode, output),
    };
}

pub(crate) fn app() {
    let application = Application::from_args();

    match application {
        Application::Schema { .. } => typescript(application.clone()),
    }
}
