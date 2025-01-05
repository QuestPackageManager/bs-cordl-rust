#[cfg(feature = "BeatSaber+BeatAvatarSDK+HeadBodyOffsetSO")]
#[repr(C)]
#[derive(Debug)]
pub struct HeadBodyOffsetSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _headNeckOffset: crate::UnityEngine::Vector3,
    pub _verticalOffset: f32,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+HeadBodyOffsetSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::BeatAvatarSDK::HeadBodyOffsetSO =>
    "BeatSaber.BeatAvatarSDK"."HeadBodyOffsetSO"
);
#[cfg(feature = "BeatSaber+BeatAvatarSDK+HeadBodyOffsetSO")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarSDK::HeadBodyOffsetSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+HeadBodyOffsetSO")]
impl std::ops::DerefMut for crate::BeatSaber::BeatAvatarSDK::HeadBodyOffsetSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+HeadBodyOffsetSO")]
impl crate::BeatSaber::BeatAvatarSDK::HeadBodyOffsetSO {
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
    pub fn get_headNeckOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_headNeckOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_verticalOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_verticalOffset", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+HeadBodyOffsetSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarSDK::HeadBodyOffsetSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
