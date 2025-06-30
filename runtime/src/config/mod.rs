use frame_support::parameter_types;
// Local module imports
use super::{OriginCaller, Runtime, RuntimeCall, RuntimeEvent};

// Configure utility pallet.
impl pallet_utility::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type PalletsOrigin = OriginCaller;
    type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
}

// Define counter max value runtime constant.
parameter_types! {
    pub const CounterMaxValue: u32 = 500;
}

// Configure custom pallet.
impl custom_pallet::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type CounterMaxValue = CounterMaxValue;
    type WeightInfo = custom_pallet::weights::SubstrateWeight<Runtime>;
}