use std::ops::Deref;

use airswap::ProtocolVersion;
use alloy::primitives::Address;
use clap::{Args, Parser, Subcommand};

use crate::actions::{
    Action, GetMakersAction, GetPricingAction, GetProtocolsAction, GetTokensAction, QuoteAction,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[command(flatten)]
    pub config: Config,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Registry(RegistryCommands),
    Maker {
        address: String,
        #[command(subcommand)]
        command: MakerCommands,
    },
    Buy(BuyCommand),
    Sell(SellCommand),
}

#[derive(Subcommand)]
pub enum RegistryCommands {
    Makers,
}

#[derive(Clone, Subcommand)]
pub enum MakerCommands {
    Protocols,
    Tokens,
    Pricing {
        from_symbol: String,
        to_symbol: String,
    },
}

#[derive(Args, Clone)]
pub struct BuyCommand {
    #[arg(short, long)]
    pub maker: Option<Address>,
    pub from_symbol: String,
    pub to_amount: f64,
    pub to_symbol: String,
}

#[derive(Args, Clone)]
pub struct SellCommand {
    #[arg(short, long)]
    pub maker: Option<Address>,
    pub from_amount: f64,
    pub from_symbol: String,
    pub to_symbol: String,
}

#[derive(Args, Clone)]
pub struct Config {
    #[arg(short = 'v', long, default_value_t)]
    pub protocol_version: ProtocolVersion,

    #[arg(long, env)]
    pub rpc: String,
}

pub struct BoxedAction(Box<dyn Action>);

impl Deref for BoxedAction {
    type Target = dyn Action;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl From<&Cli> for BoxedAction {
    fn from(cli: &Cli) -> Self {
        match &cli.command {
            Commands::Registry(command) => match command {
                RegistryCommands::Makers => {
                    BoxedAction(Box::new(GetMakersAction::new(cli.config.clone())))
                }
            },
            Commands::Maker { address, command } => match command {
                MakerCommands::Protocols => BoxedAction(Box::new(GetProtocolsAction::new(
                    cli.config.clone(),
                    address.clone(),
                ))),
                MakerCommands::Tokens => BoxedAction(Box::new(GetTokensAction::new(
                    cli.config.clone(),
                    address.clone(),
                ))),
                MakerCommands::Pricing {
                    from_symbol,
                    to_symbol,
                } => BoxedAction(Box::new(GetPricingAction::new(
                    cli.config.clone(),
                    address.clone(),
                    from_symbol.clone(),
                    to_symbol.clone(),
                ))),
            },
            Commands::Buy(command) => BoxedAction(Box::new(QuoteAction::buy(
                cli.config.clone(),
                command.clone(),
            ))),
            Commands::Sell(command) => BoxedAction(Box::new(QuoteAction::sell(
                cli.config.clone(),
                command.clone(),
            ))),
        }
    }
}
