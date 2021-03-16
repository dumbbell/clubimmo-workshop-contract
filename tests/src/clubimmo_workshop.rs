use casper_engine_test_support::{
    Code, Hash, SessionBuilder, TestContext, TestContextBuilder
};
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes},
    runtime_args,
    CLTyped,
    RuntimeArgs,
    U512,
    PublicKey,
    SecretKey
};

pub const TEST_ACCOUNT: [u8; 32] = [7u8; 32];
pub const TEST_ADDRESS: [u8; 32] = [8u8; 32];
pub const TEST_ACCOUNT_HASH: AccountHash = AccountHash::new(TEST_ADDRESS);
pub const CLUBIMMO_WORKSHOP: &str = "clubimmo_workshop_contract";
pub const CLUBIMMO_WORKSHOP_HASH: &str = "clubimmo_workshop_contract_hash";

pub struct ClubimmoWorkshopContract {
    pub context: TestContext,
    pub clubimmo_workshop_hash: Hash,
}

impl ClubimmoWorkshopContract {
    pub fn deploy() -> Self {
        let test_account_public_key: PublicKey =
            SecretKey::ed25519(TEST_ACCOUNT).into();
        let mut context = TestContextBuilder::new()
            .with_public_key(
                test_account_public_key,
                TEST_ACCOUNT_HASH,
                U512::from(128_000_000_000_000u64))
            .build();
        let session_code = Code::from("contract.wasm");
        let session = SessionBuilder::new(session_code, runtime_args! {})
            .with_address(TEST_ACCOUNT_HASH)
            .with_authorization_keys(&[TEST_ACCOUNT_HASH])
            .build();
        context.run(session);
        let clubimmo_workshop_hash =
            Self::contract_hash(&context, CLUBIMMO_WORKSHOP_HASH);
        Self {
            context,
            clubimmo_workshop_hash,
        }
    }

    pub fn contract_hash(context: &TestContext, name: &str) -> Hash {
        context
            .query(TEST_ACCOUNT_HASH, &[name.to_string()])
            .unwrap_or_else(|_| panic!("{} contract not found", name))
            .into_t()
            .unwrap_or_else(|_| panic!("{} is not a type Contract.", name))
    }

    pub fn call_new_building(&mut self, address: String, asked_price: u64) {
        let code = Code::Hash(
            self.clubimmo_workshop_hash, "new_building".to_string());
        let args = runtime_args! {
            "address" => address,
            "asked_price" => asked_price,
        };
        let session = SessionBuilder::new(code, args)
            .with_address(TEST_ACCOUNT_HASH)
            .with_authorization_keys(&[TEST_ACCOUNT_HASH])
            .build();
        self.context.run(session);
    }

    /*
    pub fn call_make_offer(&mut self, address: String, offered_price: u64) {
        let code = Code::Hash(
            self.clubimmo_workshop_hash, "make_offer".to_string());
        let args = runtime_args! {
            "address" => address,
            "offered_price" => offered_price,
        };
        let session = SessionBuilder::new(code, args)
            .with_address(TEST_ACCOUNT_HASH)
            .with_authorization_keys(&[TEST_ACCOUNT_HASH])
            .build();
        self.context.run(session);
    }
*/

    pub fn query_contract<T: CLTyped + FromBytes>(&self, address: &str) -> Option<T> {
        match self
            .context
            .query(TEST_ACCOUNT_HASH,
                &[CLUBIMMO_WORKSHOP.to_string(), address.to_string()])
        {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", address));
                Some(value)
            }
        }
    }
}
