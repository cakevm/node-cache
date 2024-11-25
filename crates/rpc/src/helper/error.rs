use jsonrpsee::types::error::INTERNAL_ERROR_CODE;
use jsonrpsee::types::{ErrorObject, ErrorObjectOwned};

pub fn alloy_error(err: alloy_transport::TransportError) -> ErrorObjectOwned {
    ErrorObject::owned(INTERNAL_ERROR_CODE, format!("Provider error: {:?}", err), None::<bool>)
}

pub fn eyre_error(err: eyre::Report) -> ErrorObjectOwned {
    ErrorObject::owned(INTERNAL_ERROR_CODE, format!("Provider error: {:?}", err), None::<bool>)
}
