#[cfg(feature = "HoudiniEngineUnity+HEU_DetailPrototype")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_DetailPrototype {
    __cordl_parent: crate::System::Object,
    pub _prototypePrefab: *mut crate::System::String,
    pub _prototypeTexture: *mut crate::System::String,
    pub _bendFactor: f32,
    pub _dryColor: crate::UnityEngine::Color,
    pub _healthyColor: crate::UnityEngine::Color,
    pub _maxHeight: f32,
    pub _maxWidth: f32,
    pub _minHeight: f32,
    pub _minWidth: f32,
    pub _noiseSpread: f32,
    pub _renderMode: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_DetailPrototype")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_DetailPrototype =>
    "HoudiniEngineUnity"."HEU_DetailPrototype"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_DetailPrototype")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_DetailPrototype {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_DetailPrototype")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_DetailPrototype {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_DetailPrototype")]
impl crate::HoudiniEngineUnity::HEU_DetailPrototype {
    pub fn IsEquivalentTo(
        &mut self,
        other: *mut crate::HoudiniEngineUnity::HEU_DetailPrototype,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret)
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_DetailPrototype")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_DetailPrototype {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
