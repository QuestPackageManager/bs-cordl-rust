#[cfg(feature = "UnityEngine+AudioReverbPreset")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioReverbPreset {
    #[default]
    Alley = 15i32,
    Arena = 10i32,
    Auditorium = 7i32,
    Bathroom = 4i32,
    CarpetedHallway = 12i32,
    Cave = 9i32,
    City = 17i32,
    Concerthall = 8i32,
    Dizzy = 25i32,
    Drugged = 24i32,
    Forest = 16i32,
    Generic = 1i32,
    Hallway = 13i32,
    Hangar = 11i32,
    Livingroom = 5i32,
    Mountains = 18i32,
    Off = 0i32,
    PaddedCell = 2i32,
    ParkingLot = 21i32,
    Plain = 20i32,
    Psychotic = 26i32,
    Quarry = 19i32,
    Room = 3i32,
    SewerPipe = 22i32,
    StoneCorridor = 14i32,
    Stoneroom = 6i32,
    Underwater = 23i32,
    User = 27i32,
}
#[cfg(feature = "UnityEngine+AudioReverbPreset")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AudioReverbPreset => "UnityEngine"
    ."AudioReverbPreset"
);
