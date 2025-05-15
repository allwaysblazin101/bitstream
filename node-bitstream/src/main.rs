use sc_cli::{RunCmd, SubstrateCli};
use node_bitstream::cli::{Cli, Subcommand};

fn main() -> sc_cli::Result<()> {
    let cli = Cli::from_args();
    match &cli.subcommand {
        Some(Subcommand::Key(cmd)) => cmd.run(),
        None => {
            let runner = cli.create_runner(&cli.run)?;
            runner.run_node_until_exit(|config| async move {
                node_bitstream::service::new_full(config).await
            })
        }
    }
}
