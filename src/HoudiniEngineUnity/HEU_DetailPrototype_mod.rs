#[cfg(feature = "HoudiniEngineUnity+HEU_DetailPrototype")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_DetailPrototype {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _prototypePrefab: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _prototypeTexture: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        other: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_DetailPrototype>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
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
#[cfg(feature = "HoudiniEngineUnity+HEU_DetailPrototype")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_DetailPrototype>,
    >,
> for crate::HoudiniEngineUnity::HEU_DetailPrototype {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_DetailPrototype>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_DetailPrototype")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_DetailPrototype>,
    >,
> for crate::HoudiniEngineUnity::HEU_DetailPrototype {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_DetailPrototype>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
