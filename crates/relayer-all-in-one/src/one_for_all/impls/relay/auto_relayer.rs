use async_trait::async_trait;

use crate::one_for_all::traits::relay::{OfaBaseRelay, OfaRelayPreset};
use crate::one_for_all::types::relay::OfaRelayWrapper;
use crate::std_prelude::*;
use ibc_relayer_components::relay::traits::auto_relayer::{AutoRelayer, CanAutoRelay};

#[async_trait]
impl<Relay, Preset> CanAutoRelay for OfaRelayWrapper<Relay>
where
    Relay: OfaBaseRelay<Preset = Preset>,
    Preset: OfaRelayPreset<Relay>,
{
    async fn auto_relay(&self) {
        Preset::AutoRelayer::auto_relay(self).await
    }
}
