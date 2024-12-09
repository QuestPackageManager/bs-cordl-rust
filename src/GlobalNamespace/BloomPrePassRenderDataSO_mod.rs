#[cfg(feature = "BloomPrePassRenderDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassRenderDataSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub data: *mut crate::GlobalNamespace::BloomPrePassRenderDataSO_Data,
}
#[cfg(feature = "BloomPrePassRenderDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BloomPrePassRenderDataSO => ""
    ."BloomPrePassRenderDataSO"
);
#[cfg(feature = "BloomPrePassRenderDataSO")]
impl std::ops::Deref for crate::GlobalNamespace::BloomPrePassRenderDataSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassRenderDataSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::BloomPrePassRenderDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassRenderDataSO")]
impl crate::GlobalNamespace::BloomPrePassRenderDataSO {
    #[cfg(feature = "BloomPrePassRenderDataSO+Data")]
    pub type Data = crate::GlobalNamespace::BloomPrePassRenderDataSO_Data;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BloomPrePassRenderDataSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassRenderDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BloomPrePassRenderDataSO+Data")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassRenderDataSO_Data {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub bloomPrePassRenderTexture: *mut crate::UnityEngine::RenderTexture,
    pub textureToScreenRatio: crate::UnityEngine::Vector2,
    pub viewMatrix: crate::UnityEngine::Matrix4x4,
    pub projectionMatrix: crate::UnityEngine::Matrix4x4,
    pub stereoCameraEyeOffset: f32,
    pub toneMapping: crate::GlobalNamespace::ToneMapping,
}
#[cfg(feature = "BloomPrePassRenderDataSO+Data")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BloomPrePassRenderDataSO_Data
    => ""."BloomPrePassRenderDataSO/Data"
);
#[cfg(feature = "BloomPrePassRenderDataSO+Data")]
impl std::ops::Deref for crate::GlobalNamespace::BloomPrePassRenderDataSO_Data {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassRenderDataSO+Data")]
impl std::ops::DerefMut for crate::GlobalNamespace::BloomPrePassRenderDataSO_Data {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassRenderDataSO+Data")]
impl crate::GlobalNamespace::BloomPrePassRenderDataSO_Data {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BloomPrePassRenderDataSO+Data")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassRenderDataSO_Data {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
