use std::path::PathBuf;

use clap::Parser;
use eyre::Result;
use openvm_sdk::{
    fs::{
        read_app_proof_from_file, read_app_vk_from_file, read_evm_proof_from_file,
        read_evm_verifier_from_folder,
    },
    Sdk,
};

use crate::default::{
    DEFAULT_APP_PROOF_PATH, DEFAULT_APP_VK_PATH, DEFAULT_EVM_PROOF_PATH, DEFAULT_VERIFIER_FOLDER,
};

#[derive(Parser)]
#[command(name = "verify", about = "Verify a proof")]
pub struct VerifyCmd {
    #[command(subcommand)]
    command: VerifySubCommand,
}

#[derive(Parser)]
enum VerifySubCommand {
    App {
        #[arg(long, action, help = "Path to app verifying key", default_value = DEFAULT_APP_VK_PATH)]
        app_vk: PathBuf,

        #[arg(long, action, help = "Path to app proof", default_value = DEFAULT_APP_PROOF_PATH)]
        proof: PathBuf,
    },
    Evm {
        #[arg(long, action, help = "Path to EVM proof", default_value = DEFAULT_EVM_PROOF_PATH)]
        proof: PathBuf,
    },
}

impl VerifyCmd {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            VerifySubCommand::App { app_vk, proof } => {
                let app_vk = read_app_vk_from_file(app_vk)?;
                let app_proof = read_app_proof_from_file(proof)?;
                Sdk.verify_app_proof(&app_vk, &app_proof)?;
            }
            VerifySubCommand::Evm { proof } => {
                let evm_verifier = read_evm_verifier_from_folder(DEFAULT_VERIFIER_FOLDER).map_err(|e| {
                    eyre::eyre!("Failed to read EVM verifier: {}\nPlease run 'cargo openvm evm-proving-setup' first", e)
                })?;
                let evm_proof = read_evm_proof_from_file(proof)?;
                Sdk.verify_evm_proof(&evm_verifier, &evm_proof)?;
            }
        }
        Ok(())
    }
}
