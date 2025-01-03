#[cfg(feature = "UnityEngine+LightmapData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightmapData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Light: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    pub m_Dir: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    pub m_ShadowMask: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
}
#[cfg(feature = "UnityEngine+LightmapData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LightmapData => "UnityEngine"
    ."LightmapData"
);
#[cfg(feature = "UnityEngine+LightmapData")]
impl std::ops::Deref for crate::UnityEngine::LightmapData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LightmapData")]
impl std::ops::DerefMut for crate::UnityEngine::LightmapData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LightmapData")]
impl crate::UnityEngine::LightmapData {
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
#[cfg(feature = "UnityEngine+LightmapData")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::LightmapData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
