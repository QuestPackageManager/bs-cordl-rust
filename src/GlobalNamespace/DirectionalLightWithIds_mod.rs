#[cfg(feature = "DirectionalLightWithIds")]
#[repr(C)]
#[derive(Debug)]
pub struct DirectionalLightWithIds {
    __cordl_parent: crate::GlobalNamespace::RuntimeLightWithIds,
    pub _directionalLight: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DirectionalLight,
    >,
    pub _setIntensityOnly: bool,
    pub _defaultColor: crate::UnityEngine::Color,
}
#[cfg(feature = "DirectionalLightWithIds")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DirectionalLightWithIds => ""
    ."DirectionalLightWithIds"
);
#[cfg(feature = "DirectionalLightWithIds")]
impl std::ops::Deref for crate::GlobalNamespace::DirectionalLightWithIds {
    type Target = crate::GlobalNamespace::RuntimeLightWithIds;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DirectionalLightWithIds")]
impl std::ops::DerefMut for crate::GlobalNamespace::DirectionalLightWithIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DirectionalLightWithIds")]
impl crate::GlobalNamespace::DirectionalLightWithIds {
    pub fn ColorWasSet(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ColorWasSet", (color))?;
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
#[cfg(feature = "DirectionalLightWithIds")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DirectionalLightWithIds {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
