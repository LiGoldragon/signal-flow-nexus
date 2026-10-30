#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Launch_Data {
    pub address: flow_nexus_types::Address,
    pub remembering: flow_nexus_types::Remembering,
    pub brief: flow_nexus_types::Brief,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Send_Data {
    pub address: flow_nexus_types::Address,
    pub request: flow_nexus_types::Request,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Launch(Launch_Data),
    End(flow_nexus_types::FlowId),
    Send(Send_Data),
    Refresh(flow_nexus_types::FlowId),
    Hook(flow_nexus_types::RecordHook),
    Configure(flow_nexus_types::Configuration),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Refused_Data {
    NoCapsule,
    RouteClosed,
    AddressBusy,
    UndeclaredAddress,
    UnknownFlow,
    HarnessRefused,
    PermissionRequired,
    StoreRefused,
    RecoveryRequired,
    Unavailable,
    ConfigurationRefused,
    ConfigureClosed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Launched(flow_nexus_types::FlowId),
    Ended,
    Delivered,
    Queued,
    Started(flow_nexus_types::FlowId),
    Refreshed(flow_nexus_types::FlowId),
    Recorded,
    Configured,
    Refused(Refused_Data),
}
