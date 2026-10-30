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
pub struct Hook_Data {
    pub flow_id: flow_nexus_types::FlowId,
    pub session_id_option: std::option::Option<flow_nexus_types::SessionId>,
    pub state: flow_nexus_types::State,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Launch(Launch_Data),
    End(flow_nexus_types::FlowId),
    Send(Send_Data),
    Refresh(flow_nexus_types::FlowId),
    Hook(Hook_Data),
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
    Refused(Refused_Data),
}
