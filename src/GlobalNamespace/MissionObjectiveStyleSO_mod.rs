#[cfg(feature = "MissionObjectiveStyleSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionObjectiveStyleSO {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub titleTextStyle: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::TextStyleSO>,
    pub titleColorStyle: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorStyleSO>,
    pub backgroundColorStyle: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorStyleSO,
    >,
}
#[cfg(feature = "MissionObjectiveStyleSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionObjectiveStyleSO => ""
    ."MissionObjectiveStyleSO"
);
#[cfg(feature = "MissionObjectiveStyleSO")]
impl std::ops::Deref for crate::GlobalNamespace::MissionObjectiveStyleSO {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjectiveStyleSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionObjectiveStyleSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjectiveStyleSO")]
impl crate::GlobalNamespace::MissionObjectiveStyleSO {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MissionObjectiveStyleSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionObjectiveStyleSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
