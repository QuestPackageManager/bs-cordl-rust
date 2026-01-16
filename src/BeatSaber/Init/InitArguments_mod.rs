#[cfg(feature = "cordl_class_BeatSaber+Init+InitArguments")]
#[repr(C)]
#[derive(Debug)]
pub struct InitArguments {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BeatSaber+Init+InitArguments")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::Init::InitArguments {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Init";
    const CLASS_NAME: &'static str = "InitArguments";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BeatSaber+Init+InitArguments")]
impl std::ops::Deref for crate::BeatSaber::Init::InitArguments {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+InitArguments")]
impl std::ops::DerefMut for crate::BeatSaber::Init::InitArguments {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+InitArguments")]
impl crate::BeatSaber::Init::InitArguments {
    pub const kAutoPlayOptionIdentifier: &'static str = "--auto_play";
    pub const kAutoRecOptionIdentifier: &'static str = "--auto_rec";
    pub const kCustomSettingsOptionIdentifier: &'static str = "--customSettings";
    pub const kRecordingToolOptionIdentifier: &'static str = "--enable_recording_tool";
}
#[cfg(feature = "cordl_class_BeatSaber+Init+InitArguments")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::Init::InitArguments {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
