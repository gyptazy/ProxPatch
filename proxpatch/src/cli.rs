use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "ProxPatch",
    version,
    about = "An automated security patch tool for Proxmox clusters.\nAuthor: Florian Paul Azim Hoberg @gyptazy <contact@gyptazy.com>"
)]

pub struct Cli {
    #[arg(short = 'd', long, help = "Run ProxPatch in debug mode")]
    pub debug: bool,
}
