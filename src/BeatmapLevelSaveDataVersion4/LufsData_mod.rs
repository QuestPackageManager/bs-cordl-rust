#[cfg(feature = "BeatmapLevelSaveDataVersion4+LufsData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LufsData {
    pub si: i32,
    pub ei: i32,
    pub l: f32,
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+LufsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapLevelSaveDataVersion4::LufsData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatmapLevelSaveDataVersion4";
    const CLASS_NAME: &'static str = "LufsData";
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
#[cfg(feature = "BeatmapLevelSaveDataVersion4+LufsData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatmapLevelSaveDataVersion4::LufsData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+LufsData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatmapLevelSaveDataVersion4::LufsData {
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
#[cfg(feature = "BeatmapLevelSaveDataVersion4+LufsData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatmapLevelSaveDataVersion4::LufsData {
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
#[cfg(feature = "BeatmapLevelSaveDataVersion4+LufsData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatmapLevelSaveDataVersion4::LufsData {
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
#[cfg(feature = "BeatmapLevelSaveDataVersion4+LufsData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapLevelSaveDataVersion4::LufsData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+LufsData")]
impl crate::BeatmapLevelSaveDataVersion4::LufsData {}
