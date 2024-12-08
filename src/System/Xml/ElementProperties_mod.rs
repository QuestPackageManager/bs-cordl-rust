#[cfg(feature = "System+Xml+ElementProperties")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementProperties {
    BLOCK_WS = 8421440u32,
    BOOL_PARENT = 268960770u32,
    DEFAULT = 67240192u32,
    EMPTY = 1075843080u32,
    HAS_NS = 16810112u32,
    HEAD = 2155888672u32,
    NAME_PARENT = 537921540u32,
    NO_ENTITIES = 2151686160u32,
    URI_PARENT = 134480385u32,
}
#[cfg(feature = "System+Xml+ElementProperties")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::ElementProperties => "System.Xml"
    ."ElementProperties"
);
