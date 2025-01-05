#[cfg(feature = "LayerMasks")]
#[repr(C)]
#[derive(Debug)]
pub struct LayerMasks {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LayerMasks")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LayerMasks => ""."LayerMasks"
);
#[cfg(feature = "LayerMasks")]
impl std::ops::Deref for crate::GlobalNamespace::LayerMasks {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LayerMasks")]
impl std::ops::DerefMut for crate::GlobalNamespace::LayerMasks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LayerMasks")]
impl crate::GlobalNamespace::LayerMasks {
    pub fn GetLayer(
        layerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLayer", (layerName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLayerMask_Il2CppString0(
        layerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_ret: crate::UnityEngine::LayerMask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLayerMask", (layerName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLayerMask_i32_1(
        layerNum: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_ret: crate::UnityEngine::LayerMask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLayerMask", (layerNum))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "LayerMasks")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LayerMasks {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
