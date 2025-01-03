#[cfg(feature = "MissionStage")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionStage {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _minimumMissionsToUnlock: i32,
    pub _rectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
}
#[cfg(feature = "MissionStage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionStage => ""
    ."MissionStage"
);
#[cfg(feature = "MissionStage")]
impl std::ops::Deref for crate::GlobalNamespace::MissionStage {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionStage")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionStage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionStage")]
impl crate::GlobalNamespace::MissionStage {
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
    pub fn get_minimumMissionsToUnlock(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_minimumMissionsToUnlock", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_position", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MissionStage")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MissionStage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
