use std::collections::HashMap;

extern crate wasm;

use wasm::ext::{External, Result as ExtResult, Error as ExtError};

extern crate byteorder;

extern crate primitives;
use primitives::types::{AccountId, PromiseId, ReceiptId, Mana, Balance};

#[derive(Default)]
struct MyExt {
    storage: HashMap<Vec<u8>, Vec<u8>>,
    num_receipts: u32,
}

fn generate_promise_id(index: u32) -> ReceiptId {
    [index as u8; 32].to_vec()
}

impl External for MyExt {
    fn storage_set(&mut self, key: &[u8], value: &[u8]) -> ExtResult<()> {
        println!("PUT '{:?}' -> '{:?}'", key, value);
        self.storage.insert(Vec::from(key), Vec::from(value));
        Ok(())
    }

    fn storage_get(&self, key: &[u8]) -> ExtResult<Option<Vec<u8>>> {
        let value = self.storage.get(key);
        match value {
            Some(buf) => {
                println!("GET '{:?}' -> '{:?}'", key, buf);
                Ok(Some(buf.to_vec()))
            }
            None => {
                println!("GET '{:?}' -> EMPTY", key);
                Ok(None)
            }
        }
    }

    fn promise_create(
        &mut self,
        account_id: AccountId,
        _method_name: Vec<u8>,
        _arguments: Vec<u8>,
        _mana: Mana,
        _amount: Balance,
    ) -> ExtResult<PromiseId> {
        match self.num_receipts {
            0 => assert_eq!(&account_id, &AccountId::from(&"test1".to_string())),
            1 => assert_eq!(&account_id, &AccountId::from(&"test2".to_string())),
            _ => (),
        };
        self.num_receipts += 1;
        Ok(PromiseId::Receipt(generate_promise_id(self.num_receipts - 1)))
    }

    fn promise_then(
        &mut self,
        promise_id: PromiseId,
        _method_name: Vec<u8>,
        _arguments: Vec<u8>,
        _mana: Mana,
    ) -> ExtResult<PromiseId> {
        match promise_id {
            PromiseId::Receipt(_) => {
                assert!(false);
                Err(ExtError::WrongPromise)
            },
            PromiseId::Joiner(v) => {
                assert_eq!(v[0], generate_promise_id(0));
                assert_eq!(v[1], generate_promise_id(1));
                Ok(PromiseId::Callback(b"call_it_please".to_vec()))
            },
            _ => Err(ExtError::WrongPromise),
        }
    }
}

#[cfg(test)]
mod tests {
    use byteorder::{ByteOrder, LittleEndian};
    use std::fs;
    use wasm::executor::{self, ExecutionOutcome};
    use wasm::types::{Error, Config, RuntimeContext, ReturnData};
    
    use super::*;

    fn run(
        method_name: &[u8],
        input_data: &[u8],
        result_data: &[Option<Vec<u8>>],
        context: &RuntimeContext,
    ) -> Result<ExecutionOutcome, Error> {
        let wasm_binary = fs::read("res/main2.wasm").expect("Unable to read file");

        let mut ext = MyExt::default();
        let mut config = Config::default();
        config.gas_limit = 1 << 20;

        executor::execute(
            &wasm_binary,
            &method_name,
            &input_data,
            &result_data,
            &mut ext,
            &config,
            &context,
        )
    }

    fn encode_i32(val: i32) -> [u8; 4] {
        let mut tmp = [0u8; 4];
        LittleEndian::write_i32(&mut tmp, val);
        tmp
    }

    fn encode_u64(val: u64) -> [u8; 8] {
        let mut tmp = [0u8; 8];
        LittleEndian::write_u64(&mut tmp, val);
        tmp
    }

    fn runtime_context(
        balance: Balance,
        amount: Balance,
        mana: Mana,
    ) -> RuntimeContext {
        RuntimeContext::new(
            balance,
            amount,
            AccountId::from(&"alice".to_string()),
            AccountId::from(&"bob".to_string()),
            mana
        )
    }

    #[test]
    fn test_storage()  {
        let input_data = [0u8; 0];
        
        let return_data = run(
            b"runTest",
            &input_data,
            &[],
            &runtime_context(0, 0, 0),
        ).map(|outcome| outcome.return_data)
        .expect("ok");
        
        match return_data {
            ReturnData::Value(output_data) => assert_eq!(&output_data, &encode_i32(20)),
            _ => assert!(false, "Expected returned value"),
        };
    }

}
