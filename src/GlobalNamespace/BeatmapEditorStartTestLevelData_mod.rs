#[cfg(feature = "BeatmapEditorStartTestLevelData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeatmapEditorStartTestLevelData {
    pub fpfc: crate::GlobalNamespace::BeatmapEditorStartTestLevelData_FpfcData,
    pub overdrawData: crate::GlobalNamespace::BeatmapEditorStartTestLevelData_OverdrawData,
}
#[cfg(feature = "BeatmapEditorStartTestLevelData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapEditorStartTestLevelData";
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
#[cfg(feature = "BeatmapEditorStartTestLevelData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatmapEditorStartTestLevelData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "BeatmapEditorStartTestLevelData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData {
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
#[cfg(feature = "BeatmapEditorStartTestLevelData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "BeatmapEditorStartTestLevelData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapEditorStartTestLevelData")]
impl crate::GlobalNamespace::BeatmapEditorStartTestLevelData {
    #[cfg(feature = "BeatmapEditorStartTestLevelData+FpfcData")]
    pub type FpfcData = crate::GlobalNamespace::BeatmapEditorStartTestLevelData_FpfcData;
    #[cfg(feature = "BeatmapEditorStartTestLevelData+OverdrawData")]
    pub type OverdrawData = crate::GlobalNamespace::BeatmapEditorStartTestLevelData_OverdrawData;
}
#[cfg(feature = "BeatmapEditorStartTestLevelData+FpfcData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeatmapEditorStartTestLevelData_FpfcData {
    pub enabled: bool,
}
#[cfg(feature = "BeatmapEditorStartTestLevelData+FpfcData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData_FpfcData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapEditorStartTestLevelData/FpfcData";
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
#[cfg(feature = "BeatmapEditorStartTestLevelData+FpfcData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData_FpfcData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatmapEditorStartTestLevelData+FpfcData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData_FpfcData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "BeatmapEditorStartTestLevelData+FpfcData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData_FpfcData {
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
#[cfg(feature = "BeatmapEditorStartTestLevelData+FpfcData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData_FpfcData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "BeatmapEditorStartTestLevelData+FpfcData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData_FpfcData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapEditorStartTestLevelData+FpfcData")]
impl crate::GlobalNamespace::BeatmapEditorStartTestLevelData_FpfcData {}
#[cfg(feature = "BeatmapEditorStartTestLevelData+OverdrawData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeatmapEditorStartTestLevelData_OverdrawData {
    pub enabled: bool,
    pub computeBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    pub audioClipFrequency: i32,
    pub samplesPerOverdrawBucket: f32,
}
#[cfg(feature = "BeatmapEditorStartTestLevelData+OverdrawData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData_OverdrawData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapEditorStartTestLevelData/OverdrawData";
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
#[cfg(feature = "BeatmapEditorStartTestLevelData+OverdrawData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData_OverdrawData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatmapEditorStartTestLevelData+OverdrawData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData_OverdrawData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "BeatmapEditorStartTestLevelData+OverdrawData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData_OverdrawData {
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
#[cfg(feature = "BeatmapEditorStartTestLevelData+OverdrawData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData_OverdrawData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "BeatmapEditorStartTestLevelData+OverdrawData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BeatmapEditorStartTestLevelData_OverdrawData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapEditorStartTestLevelData+OverdrawData")]
impl crate::GlobalNamespace::BeatmapEditorStartTestLevelData_OverdrawData {}
