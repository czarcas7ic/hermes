use crate::one_for_all::traits::birelay::OfaBiRelayTypes;
use crate::one_for_all::types::birelay::OfaBiRelayWrapper;
use ibc_relayer_components::core::traits::error::HasErrorType;

impl<BiRelay> HasErrorType for OfaBiRelayWrapper<BiRelay>
where
    BiRelay: OfaBiRelayTypes,
{
    type Error = BiRelay::Error;
}
