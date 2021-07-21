use serde::{Deserialize, Serialize};

use crate::abi::*;
use crate::constructs::*;
use crate::types::*;
use crate::*;

/// A borrowed reference to a `Tokens` bucket.
#[derive(Debug, Serialize, Deserialize)]
pub struct TokensRef {
    rid: RID,
}

impl From<RID> for TokensRef {
    fn from(rid: RID) -> Self {
        Self { rid }
    }
}

impl TokensRef {
    pub fn amount(&self) -> U256 {
        let input = GetTokensAmountInput {
            tokens: self.rid.clone(),
        };
        let output: GetTokensAmountOutput = call_kernel!(GET_TOKENS_AMOUNT, input);

        output.amount
    }

    pub fn resource(&self) -> Resource {
        let input = GetTokensResourceInput {
            tokens: self.rid.clone(),
        };
        let output: GetTokensResourceOutput = call_kernel!(GET_TOKENS_RESOURCE, input);

        output.resource.into()
    }

    pub fn destroy(self) {
        let input = ReturnTokensInput {
            reference: self.rid.clone(),
        };
        let _: ReturnTokensOutput = call_kernel!(RETURN_TOKENS, input);
    }
}
