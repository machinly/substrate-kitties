use crate as pallet_kitties;
use pallet_balances::Call as BalancesCall;
use frame_support::traits::{ConstU16, ConstU64, Currency};
use frame_support::{PalletId, parameter_types};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

pub const EXISTENTIAL_DEPOSIT: u128 = 500;
pub type Balance = u128;

use pallet_insecure_randomness_collective_flip;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Balances: pallet_balances,
		KittiesModule: pallet_kitties,
        Randomness: pallet_insecure_randomness_collective_flip,
	}
);

impl frame_system::pallet::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_kitties::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Randomness = Randomness;
    type Currency = Balances;
    type KittyPrice = KittyPrice;
    type PalletId = KittyPalletId;
}

impl pallet_insecure_randomness_collective_flip::Config for Test {}

parameter_types! {
	pub KittyPalletId: PalletId = PalletId(*b"py/kitty");
	pub KittyPrice: Balance = EXISTENTIAL_DEPOSIT * 10;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut ext: sp_io::TestExternalities = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
    ext.execute_with(|| System::set_block_number(1));
    ext
}
