#[cfg(feature = "LightGroupEditorPrefabContext")]
#[repr(C)]
#[derive(Debug)]
pub struct LightGroupEditorPrefabContext {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub lightGroups: *mut crate::System::Collections::Generic::List_1<*mut LightGroupSO>,
}
#[cfg(feature = "LightGroupEditorPrefabContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LightGroupEditorPrefabContext => ""
    ."LightGroupEditorPrefabContext"
);
#[cfg(feature = "LightGroupEditorPrefabContext")]
impl std::ops::Deref for LightGroupEditorPrefabContext {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightGroupEditorPrefabContext")]
impl std::ops::DerefMut for LightGroupEditorPrefabContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightGroupEditorPrefabContext")]
impl LightGroupEditorPrefabContext {
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
#[cfg(feature = "LightGroupEditorPrefabContext")]
impl quest_hook::libil2cpp::ObjectType for LightGroupEditorPrefabContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
