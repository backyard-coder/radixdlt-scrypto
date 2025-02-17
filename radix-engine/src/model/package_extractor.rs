use crate::constants::EXTRACT_ABI_CREDIT;
use crate::engine::NopWasmRuntime;
use crate::fee::SystemLoanFeeReserve;
use crate::model::InvokeError;
use crate::types::*;
use crate::wasm::*;

#[derive(Debug)]
pub enum ExtractAbiError {
    InvalidWasm(PrepareError),
    FailedToExportBlueprintAbi(InvokeError<WasmError>),
    AbiDecodeError(DecodeError),
    InvalidBlueprintAbi,
}

pub fn extract_abi(code: &[u8]) -> Result<HashMap<String, BlueprintAbi>, ExtractAbiError> {
    let function_exports = WasmModule::init(code)
        .and_then(WasmModule::to_bytes)
        .map_err(ExtractAbiError::InvalidWasm)?
        .1
        .into_iter()
        .filter(|s| s.ends_with("_abi"));

    let mut wasm_engine = DefaultWasmEngine::new();
    let mut wasm_instrumenter = WasmInstrumenter::new();

    let metering_params =
        WasmMeteringParams::new(InstructionCostRules::tiered(1, 5, 10, 50000), 512);
    let instrumented_code = wasm_instrumenter.instrument(code, &metering_params);
    let mut fee_reserve = SystemLoanFeeReserve::default();
    fee_reserve.credit(EXTRACT_ABI_CREDIT);
    let mut runtime: Box<dyn WasmRuntime> = Box::new(NopWasmRuntime::new(fee_reserve));
    let mut instance = wasm_engine.instantiate(&instrumented_code);
    let mut blueprints = HashMap::new();
    for method_name in function_exports {
        let rtn = instance
            .invoke_export(&method_name, &ScryptoValue::unit(), &mut runtime)
            .map_err(ExtractAbiError::FailedToExportBlueprintAbi)?;

        let abi: BlueprintAbi =
            scrypto_decode(&rtn.raw).map_err(ExtractAbiError::AbiDecodeError)?;

        if let Type::Struct { name, fields: _ } = &abi.structure {
            blueprints.insert(name.clone(), abi);
        } else {
            return Err(ExtractAbiError::InvalidBlueprintAbi);
        }
    }
    Ok(blueprints)
}
