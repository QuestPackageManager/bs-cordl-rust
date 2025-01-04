#[cfg(feature = "GameplayModifierMask")]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameplayModifierMask {
    #[default]
    All = 65535u16,
    BatteryEnergy = 1u16,
    DisappearingArrows = 128u16,
    FastNotes = 32u16,
    FasterSong = 256u16,
    GhostNotes = 2048u16,
    InstaFail = 4u16,
    MakingGameEasier = 17944u16,
    MakingGameHarder = 47584u16,
    NoArrows = 1024u16,
    NoBombs = 16u16,
    NoFail = 2u16,
    NoObstacles = 8u16,
    None = 0u16,
    ProMode = 8192u16,
    SlowerSong = 512u16,
    SmallCubes = 32768u16,
    StrictAngles = 64u16,
    SuperFastSong = 4096u16,
    ZenMode = 16384u16,
}
#[cfg(feature = "GameplayModifierMask")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayModifierMask => ""
    ."GameplayModifierMask"
);
