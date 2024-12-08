#[cfg(feature = "LightGroupElementsSpawner")]
#[repr(C)]
#[derive(Debug)]
pub struct LightGroupElementsSpawner {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _lightPrefab: *mut crate::UnityEngine::GameObject,
    pub _useAlternatePrefab: bool,
    pub _alternateLightPrefab: *mut crate::UnityEngine::GameObject,
}
#[cfg(feature = "LightGroupElementsSpawner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LightGroupElementsSpawner => ""
    ."LightGroupElementsSpawner"
);
#[cfg(feature = "LightGroupElementsSpawner")]
impl std::ops::Deref for LightGroupElementsSpawner {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightGroupElementsSpawner")]
impl std::ops::DerefMut for LightGroupElementsSpawner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightGroupElementsSpawner")]
impl LightGroupElementsSpawner {
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
#[cfg(feature = "LightGroupElementsSpawner")]
impl quest_hook::libil2cpp::ObjectType for LightGroupElementsSpawner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
