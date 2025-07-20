#[cfg(feature = "TMPro+FontAssetCreationSettings")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FontAssetCreationSettings {
    pub sourceFontFileName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub sourceFontFileGUID: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub pointSizeSamplingMode: i32,
    pub pointSize: i32,
    pub padding: i32,
    pub packingMode: i32,
    pub atlasWidth: i32,
    pub atlasHeight: i32,
    pub characterSetSelectionMode: i32,
    pub characterSequence: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub referencedFontAssetGUID: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub referencedTextAssetGUID: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub fontStyle: i32,
    pub fontStyleModifier: f32,
    pub renderMode: i32,
    pub includeFontFeatures: bool,
}
#[cfg(feature = "TMPro+FontAssetCreationSettings")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::FontAssetCreationSettings {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "FontAssetCreationSettings";
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
#[cfg(feature = "TMPro+FontAssetCreationSettings")]
unsafe impl quest_hook::libil2cpp::Argument for crate::TMPro::FontAssetCreationSettings {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "TMPro+FontAssetCreationSettings")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::TMPro::FontAssetCreationSettings {
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
#[cfg(feature = "TMPro+FontAssetCreationSettings")]
unsafe impl quest_hook::libil2cpp::Returned for crate::TMPro::FontAssetCreationSettings {
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
#[cfg(feature = "TMPro+FontAssetCreationSettings")]
unsafe impl quest_hook::libil2cpp::Return for crate::TMPro::FontAssetCreationSettings {
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
#[cfg(feature = "TMPro+FontAssetCreationSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::TMPro::FontAssetCreationSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+FontAssetCreationSettings")]
impl crate::TMPro::FontAssetCreationSettings {
    pub fn _ctor(
        &mut self,
        sourceFontFileGUID: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pointSize: i32,
        pointSizeSamplingMode: i32,
        padding: i32,
        packingMode: i32,
        atlasWidth: i32,
        atlasHeight: i32,
        characterSelectionMode: i32,
        characterSet: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        renderMode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::FontAssetCreationSettings as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                    i32,
                    i32,
                    i32,
                    i32,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                10usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::FontAssetCreationSettings as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 10usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        sourceFontFileGUID,
                        pointSize,
                        pointSizeSamplingMode,
                        padding,
                        packingMode,
                        atlasWidth,
                        atlasHeight,
                        characterSelectionMode,
                        characterSet,
                        renderMode,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
