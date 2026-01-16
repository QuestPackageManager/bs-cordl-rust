#[cfg(feature = "cordl_class_UnityEngine+AudioReverbPreset")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
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
#[cfg(feature = "cordl_class_UnityEngine+AudioReverbPreset")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::AudioReverbPreset {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "AudioReverbPreset";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+AudioReverbPreset")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::AudioReverbPreset {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+AudioReverbPreset")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::AudioReverbPreset {
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
#[cfg(feature = "cordl_class_UnityEngine+AudioReverbPreset")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::AudioReverbPreset {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+AudioReverbPreset")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::AudioReverbPreset {
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
