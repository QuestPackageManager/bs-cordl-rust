#[cfg(feature = "Zenject+SceneContextRegistryAdderAndRemover")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneContextRegistryAdderAndRemover {
    __cordl_parent: crate::System::Object,
    pub _registry: *mut crate::Zenject::SceneContextRegistry,
    pub _sceneContext: *mut crate::Zenject::SceneContext,
}
#[cfg(feature = "Zenject+SceneContextRegistryAdderAndRemover")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SceneContextRegistryAdderAndRemover =>
    "Zenject"."SceneContextRegistryAdderAndRemover"
);
#[cfg(feature = "Zenject+SceneContextRegistryAdderAndRemover")]
impl std::ops::Deref for crate::Zenject::SceneContextRegistryAdderAndRemover {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SceneContextRegistryAdderAndRemover")]
impl std::ops::DerefMut for crate::Zenject::SceneContextRegistryAdderAndRemover {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SceneContextRegistryAdderAndRemover")]
impl crate::Zenject::SceneContextRegistryAdderAndRemover {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        sceneContext: *mut crate::Zenject::SceneContext,
        registry: *mut crate::Zenject::SceneContextRegistry,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sceneContext, registry))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        sceneContext: *mut crate::Zenject::SceneContext,
        registry: *mut crate::Zenject::SceneContextRegistry,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sceneContext, registry))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+SceneContextRegistryAdderAndRemover")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SceneContextRegistryAdderAndRemover {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
