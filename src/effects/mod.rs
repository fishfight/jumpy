use serde::{Deserialize, Serialize};

pub mod active;
pub mod passive;

pub use passive::{PassiveEffectInstance, PassiveEffectMetadata};

pub use active::{ActiveEffectKind, ActiveEffectMetadata, TriggeredEffectTrigger};

/// This is used to allow both active and passive effects to be used as values in JSON
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum AnyEffectParams {
    Active(ActiveEffectMetadata),
    Passive(Box<PassiveEffectMetadata>),
}
