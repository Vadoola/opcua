// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
#[allow(unused_imports)]
use string::*;
#[allow(unused_imports)]
use data_types::*;
#[allow(unused_imports)]
use data_value::*;
#[allow(unused_imports)]
use date_time::*;
#[allow(unused_imports)]
use node_id::*;
#[allow(unused_imports)]
use service_types::enums::*;
#[allow(unused_imports)]
use variant::*;
#[allow(unused_imports)]
use service_types::impls::*;
#[allow(unused_imports)]
use node_ids::ObjectId;
#[allow(unused_imports)]
use status_codes::StatusCode;
use service_types::MonitoringParameters;

#[derive(Debug, Clone, PartialEq)]
pub struct MonitoredItemModifyRequest {
    pub monitored_item_id: UInt32,
    pub requested_parameters: MonitoringParameters,
}

impl MessageInfo for MonitoredItemModifyRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::MonitoredItemModifyRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<MonitoredItemModifyRequest> for MonitoredItemModifyRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.monitored_item_id.byte_len();
        size += self.requested_parameters.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.monitored_item_id.encode(stream)?;
        size += self.requested_parameters.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let monitored_item_id = UInt32::decode(stream)?;
        let requested_parameters = MonitoringParameters::decode(stream)?;
        Ok(MonitoredItemModifyRequest {
            monitored_item_id,
            requested_parameters,
        })
    }
}