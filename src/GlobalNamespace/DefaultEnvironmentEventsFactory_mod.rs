#[cfg(feature = "DefaultEnvironmentEventsFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultEnvironmentEventsFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "DefaultEnvironmentEventsFactory")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::DefaultEnvironmentEventsFactory {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "DefaultEnvironmentEventsFactory";
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
#[cfg(feature = "DefaultEnvironmentEventsFactory")]
impl std::ops::Deref for crate::GlobalNamespace::DefaultEnvironmentEventsFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultEnvironmentEventsFactory")]
impl std::ops::DerefMut for crate::GlobalNamespace::DefaultEnvironmentEventsFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DefaultEnvironmentEventsFactory")]
impl crate::GlobalNamespace::DefaultEnvironmentEventsFactory {
    pub fn InsertDefaultEvents(
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InsertDefaultEvents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InsertDefaultEvents", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (beatmapData))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DefaultEnvironmentEventsFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DefaultEnvironmentEventsFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
