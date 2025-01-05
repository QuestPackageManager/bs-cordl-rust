#[cfg(feature = "MissionNodeGizmos")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionNodeGizmos {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _missionNode: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
    pub _gameplayModifiersModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifiersModelSO,
    >,
    pub _missionProgressModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CampaignProgressModel,
    >,
}
#[cfg(feature = "MissionNodeGizmos")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionNodeGizmos => ""
    ."MissionNodeGizmos"
);
#[cfg(feature = "MissionNodeGizmos")]
impl std::ops::Deref for crate::GlobalNamespace::MissionNodeGizmos {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionNodeGizmos")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionNodeGizmos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionNodeGizmos")]
impl crate::GlobalNamespace::MissionNodeGizmos {
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
#[cfg(feature = "MissionNodeGizmos")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MissionNodeGizmos {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
