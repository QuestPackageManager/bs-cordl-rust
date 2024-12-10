#[cfg(feature = "Zenject+ZenAutoInjecter")]
#[repr(C)]
#[derive(Debug)]
pub struct ZenAutoInjecter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _containerSource: crate::Zenject::ZenAutoInjecter_ContainerSources,
    pub _hasInjected: bool,
}
#[cfg(feature = "Zenject+ZenAutoInjecter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ZenAutoInjecter => "Zenject"
    ."ZenAutoInjecter"
);
#[cfg(feature = "Zenject+ZenAutoInjecter")]
impl std::ops::Deref for crate::Zenject::ZenAutoInjecter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenAutoInjecter")]
impl std::ops::DerefMut for crate::Zenject::ZenAutoInjecter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenAutoInjecter")]
impl crate::Zenject::ZenAutoInjecter {
    #[cfg(feature = "Zenject+ZenAutoInjecter+ContainerSources")]
    pub type ContainerSources = crate::Zenject::ZenAutoInjecter_ContainerSources;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Construct(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Construct", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContainerForCurrentScene(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = __cordl_object
            .invoke("GetContainerForCurrentScene", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LookupContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = __cordl_object
            .invoke("LookupContainer", ())?;
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
    pub fn get_ContainerSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Zenject::ZenAutoInjecter_ContainerSources,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Zenject::ZenAutoInjecter_ContainerSources = __cordl_object
            .invoke("get_ContainerSource", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ContainerSource(
        &mut self,
        value: crate::Zenject::ZenAutoInjecter_ContainerSources,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ContainerSource", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+ZenAutoInjecter")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ZenAutoInjecter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+ZenAutoInjecter+ContainerSources")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZenAutoInjecter_ContainerSources {
    ProjectContext = 1i32,
    SceneContext = 0i32,
    SearchHierarchy = 2i32,
}
#[cfg(feature = "Zenject+ZenAutoInjecter+ContainerSources")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ZenAutoInjecter_ContainerSources =>
    "Zenject"."ZenAutoInjecter/ContainerSources"
);
