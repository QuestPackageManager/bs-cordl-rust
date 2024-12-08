#[cfg(feature = "MidiParser+MetaEventType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetaEventType {
    Tempo = 81u8,
}
#[cfg(feature = "MidiParser+MetaEventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::MidiParser::MetaEventType => "MidiParser"
    ."MetaEventType"
);
