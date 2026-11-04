#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Define(signal_flow_nexus::Complex),
    Describe(signal_flow_nexus::ComplexName),
    Definitions,
    Launch(signal_flow_nexus::LaunchRequest),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Defined(signal_flow_nexus::ComplexName),
    Described(signal_flow_nexus::Complex),
    Listed(std::vec::Vec<signal_flow_nexus::ComplexName>),
    Refused(signal_flow_nexus::RequestFailure),
    Launched(signal_flow_nexus::FlowId),
}
