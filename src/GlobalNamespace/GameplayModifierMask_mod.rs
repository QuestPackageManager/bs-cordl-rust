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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameplayModifierMask {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameplayModifierMask";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "GameplayModifierMask")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::GameplayModifierMask {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "GameplayModifierMask")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::GameplayModifierMask {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "GameplayModifierMask")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::GameplayModifierMask {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "GameplayModifierMask")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::GameplayModifierMask {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
