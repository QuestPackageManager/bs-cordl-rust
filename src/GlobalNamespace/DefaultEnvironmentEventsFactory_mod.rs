#[cfg(feature = "DefaultEnvironmentEventsFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultEnvironmentEventsFactory {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "DefaultEnvironmentEventsFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DefaultEnvironmentEventsFactory
    => ""."DefaultEnvironmentEventsFactory"
);
#[cfg(feature = "DefaultEnvironmentEventsFactory")]
impl std::ops::Deref for crate::GlobalNamespace::DefaultEnvironmentEventsFactory {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InsertDefaultEvents", (beatmapData))?;
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
