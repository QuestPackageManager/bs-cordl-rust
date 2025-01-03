#[cfg(feature = "MissionDataExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionDataExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MissionDataExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionDataExtensions => ""
    ."MissionDataExtensions"
);
#[cfg(feature = "MissionDataExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::MissionDataExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionDataExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionDataExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionDataExtensions")]
impl crate::GlobalNamespace::MissionDataExtensions {
    pub fn Name(
        comparisonType: crate::GlobalNamespace::MissionObjective_ReferenceValueComparisonType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Name", (comparisonType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MissionDataExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionDataExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
