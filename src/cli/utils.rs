/// Common trait for all cli commands
pub trait CmdSync: clap::Parser + Sized {
    type Output;

    fn run(self) -> Self::Output;
}

pub trait CmdAsync: clap::Parser + Sized {
    type Output;

    async fn run(self) -> Self::Output;
}