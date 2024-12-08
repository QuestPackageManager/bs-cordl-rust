#[cfg(feature = "ColorArrayLightWithIdsGroupEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorArrayLightWithIdsGroupEntry {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _colorArrayLightWithIds: *mut crate::GlobalNamespace::ColorArrayLightWithIds,
    pub _lightGroups: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::LightGroupSO,
    >,
    pub _excludedLightIds: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
}
#[cfg(feature = "ColorArrayLightWithIdsGroupEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ColorArrayLightWithIdsGroupEntry => ""
    ."ColorArrayLightWithIdsGroupEntry"
);
#[cfg(feature = "ColorArrayLightWithIdsGroupEntry")]
impl std::ops::Deref for crate::GlobalNamespace::ColorArrayLightWithIdsGroupEntry {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorArrayLightWithIdsGroupEntry")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorArrayLightWithIdsGroupEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorArrayLightWithIdsGroupEntry")]
impl crate::GlobalNamespace::ColorArrayLightWithIdsGroupEntry {
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
    pub fn get_lightGroups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::LightGroupSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::LightGroupSO,
        > = __cordl_object.invoke("get_lightGroups", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ColorArrayLightWithIdsGroupEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ColorArrayLightWithIdsGroupEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
