#[cfg(feature = "BasicBeatmapEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BasicBeatmapEventType {
    #[default]
    BpmChange = 100i32,
    Event0 = 0i32,
    Event1 = 1i32,
    Event10 = 10i32,
    Event11 = 11i32,
    Event12 = 12i32,
    Event13 = 13i32,
    Event14 = 14i32,
    Event15 = 15i32,
    Event16 = 16i32,
    Event17 = 17i32,
    Event18 = 18i32,
    Event19 = 19i32,
    Event2 = 2i32,
    Event20 = 20i32,
    Event21 = 21i32,
    Event3 = 3i32,
    Event4 = 4i32,
    Event5 = 5i32,
    Event6 = 6i32,
    Event7 = 7i32,
    Event8 = 8i32,
    Event9 = 9i32,
    NoteJumpMovementSpeedChange = 1000i32,
    Special0 = 40i32,
    Special1 = 41i32,
    Special2 = 42i32,
    Special3 = 43i32,
    VoidEvent = -1i32,
}
#[cfg(feature = "BasicBeatmapEventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BasicBeatmapEventType => ""
    ."BasicBeatmapEventType"
);
