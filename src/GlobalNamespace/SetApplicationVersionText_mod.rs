#[cfg(feature = "SetApplicationVersionText")]
#[repr(C)]
#[derive(Debug)]
pub struct SetApplicationVersionText {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _versionText: *mut crate::TMPro::TextMeshPro,
    pub _labelText: *mut crate::TMPro::TextMeshPro,
    pub _buildIdText: *mut crate::TMPro::TextMeshPro,
    pub _platformIdText: *mut crate::TMPro::TextMeshPro,
    pub _footSprite: *mut crate::UnityEngine::SpriteRenderer,
    pub _rcBuildColor: crate::UnityEngine::Color,
    pub _nonRCBuildColor: crate::UnityEngine::Color,
    pub _gameVersionProvider: *mut crate::BeatSaber::Init::GameVersionProvider,
}
#[cfg(feature = "SetApplicationVersionText")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SetApplicationVersionText => ""
    ."SetApplicationVersionText"
);
#[cfg(feature = "SetApplicationVersionText")]
impl std::ops::Deref for crate::GlobalNamespace::SetApplicationVersionText {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SetApplicationVersionText")]
impl std::ops::DerefMut for crate::GlobalNamespace::SetApplicationVersionText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SetApplicationVersionText")]
impl crate::GlobalNamespace::SetApplicationVersionText {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetText(
        &mut self,
        gameVersion: quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (gameVersion))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "SetApplicationVersionText")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SetApplicationVersionText {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
