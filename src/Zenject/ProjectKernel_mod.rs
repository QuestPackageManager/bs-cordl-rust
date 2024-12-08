#[cfg(feature = "Zenject+ProjectKernel")]
#[repr(C)]
#[derive(Debug)]
pub struct ProjectKernel {
    __cordl_parent: crate::Zenject::MonoKernel,
    pub _settings: *mut crate::Zenject::ZenjectSettings,
    pub _contextRegistry: *mut crate::Zenject::SceneContextRegistry,
}
#[cfg(feature = "Zenject+ProjectKernel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ProjectKernel => "Zenject"
    ."ProjectKernel"
);
#[cfg(feature = "Zenject+ProjectKernel")]
impl std::ops::Deref for crate::Zenject::ProjectKernel {
    type Target = crate::Zenject::MonoKernel;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ProjectKernel")]
impl std::ops::DerefMut for crate::Zenject::ProjectKernel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ProjectKernel")]
impl crate::Zenject::ProjectKernel {
    #[cfg(feature = "Zenject+ProjectKernel+__c__DisplayClass4_0")]
    pub type __c__DisplayClass4_0 = crate::Zenject::ProjectKernel___c__DisplayClass4_0;
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
    pub fn OnApplicationQuit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnApplicationQuit", ())?;
        Ok(__cordl_ret)
    }
    pub fn ForceUnloadAllScenes(
        &mut self,
        immediate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ForceUnloadAllScenes", (immediate))?;
        Ok(__cordl_ret)
    }
    pub fn DestroyEverythingInOrder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyEverythingInOrder", ())?;
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
#[cfg(feature = "Zenject+ProjectKernel")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ProjectKernel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
