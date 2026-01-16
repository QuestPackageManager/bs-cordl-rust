#[cfg(feature = "cordl_class_BeatGames+Analytics+Events+FunctionalLogger")]
#[repr(C)]
#[derive(Debug)]
pub struct FunctionalLogger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BeatGames+Analytics+Events+FunctionalLogger")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatGames::Analytics::Events::FunctionalLogger {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatGames.Analytics.Events";
    const CLASS_NAME: &'static str = "FunctionalLogger";
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
#[cfg(feature = "BeatGames+Analytics+Events+FunctionalLogger")]
impl std::ops::Deref for crate::BeatGames::Analytics::Events::FunctionalLogger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatGames+Analytics+Events+FunctionalLogger")]
impl std::ops::DerefMut for crate::BeatGames::Analytics::Events::FunctionalLogger {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatGames+Analytics+Events+FunctionalLogger")]
impl crate::BeatGames::Analytics::Events::FunctionalLogger {}
#[cfg(feature = "cordl_class_BeatGames+Analytics+Events+FunctionalLogger")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatGames::Analytics::Events::FunctionalLogger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
