#[cfg(feature = "cordl_class_BeatSaber+Main+GraphQL+Enums+XOCBeatGamesBeatmapCharacteristic")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Copy", derive(Copy))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[cfg_attr(feature = "derive_Eq", derive(Eq))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[repr(i32)]
pub enum XOCBeatGamesBeatmapCharacteristic {
    #[cfg_attr(feature = "derive_Default", default)]
    DEGREE_360 = 4i32,
    DEGREE_90 = 5i32,
    EXTRA_1 = 7i32,
    EXTRA_2 = 8i32,
    LEGACY = 6i32,
    NO_ARROWS = 3i32,
    ONE_SABER = 2i32,
    STANDARD = 1i32,
    __UnknownValue = 0i32,
}
#[cfg(feature = "cordl_class_BeatSaber+Main+GraphQL+Enums+XOCBeatGamesBeatmapCharacteristic")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesBeatmapCharacteristic
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.GraphQL.Enums";
    const CLASS_NAME: &'static str = "XOCBeatGamesBeatmapCharacteristic";
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
#[cfg(feature = "cordl_class_BeatSaber+Main+GraphQL+Enums+XOCBeatGamesBeatmapCharacteristic")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesBeatmapCharacteristic
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Main+GraphQL+Enums+XOCBeatGamesBeatmapCharacteristic")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesBeatmapCharacteristic
{
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
#[cfg(feature = "cordl_class_BeatSaber+Main+GraphQL+Enums+XOCBeatGamesBeatmapCharacteristic")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesBeatmapCharacteristic
{
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
#[cfg(feature = "cordl_class_BeatSaber+Main+GraphQL+Enums+XOCBeatGamesBeatmapCharacteristic")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesBeatmapCharacteristic
{
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
