#[cfg(feature = "LevelParamsPanel")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelParamsPanel {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _notesPerSecondText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _notesCountText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _obstaclesCountText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _bombsCountText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
}
#[cfg(feature = "LevelParamsPanel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LevelParamsPanel => ""
    ."LevelParamsPanel"
);
#[cfg(feature = "LevelParamsPanel")]
impl std::ops::Deref for crate::GlobalNamespace::LevelParamsPanel {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelParamsPanel")]
impl std::ops::DerefMut for crate::GlobalNamespace::LevelParamsPanel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelParamsPanel")]
impl crate::GlobalNamespace::LevelParamsPanel {
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
    pub fn set_bombsCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bombsCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_notesCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_notesCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_notesPerSecond(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_notesPerSecond", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_obstaclesCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_obstaclesCount", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LevelParamsPanel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LevelParamsPanel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
