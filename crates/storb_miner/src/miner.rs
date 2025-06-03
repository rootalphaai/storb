use std::path::PathBuf;
use std::sync::Arc;

use base::{BaseNeuron, BaseNeuronConfig, NeuronError};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct MinerConfig {
    pub neuron_config: BaseNeuronConfig,
    pub store_dir: PathBuf,
}

/// The Storb miner.
#[derive(Clone)]
pub struct Miner {
    pub config: MinerConfig,
    pub neuron: Arc<RwLock<BaseNeuron>>,
}

impl Miner {
    pub async fn new(config: MinerConfig) -> Result<Self, NeuronError> {
        let neuron_config = config.neuron_config.clone();
        let neuron = Arc::new(RwLock::new(BaseNeuron::new(neuron_config).await?));
        let miner = Miner { config, neuron };
        Ok(miner)
    }
}
