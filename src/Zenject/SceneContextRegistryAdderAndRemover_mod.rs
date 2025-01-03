#[cfg(feature = "Zenject+SceneContextRegistryAdderAndRemover")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneContextRegistryAdderAndRemover {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        sceneContext: quest_hook::libil2cpp::Gc<crate::Zenject::SceneContext>,
        registry: quest_hook::libil2cpp::Gc<crate::Zenject::SceneContextRegistry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sceneContext, registry))?;
        Ok(__cordl_object.into())
    }
    pub fn __zenCreate(
        P_0: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("__zenCreate", (P_0))?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreateInjectTypeInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__zenCreateInjectTypeInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        sceneContext: quest_hook::libil2cpp::Gc<crate::Zenject::SceneContext>,
        registry: quest_hook::libil2cpp::Gc<crate::Zenject::SceneContextRegistry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sceneContext, registry))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Zenject+SceneContextRegistryAdderAndRemover")]
impl AsRef<crate::System::IDisposable>
for crate::Zenject::SceneContextRegistryAdderAndRemover {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+SceneContextRegistryAdderAndRemover")]
impl AsMut<crate::System::IDisposable>
for crate::Zenject::SceneContextRegistryAdderAndRemover {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+SceneContextRegistryAdderAndRemover")]
impl AsRef<crate::Zenject::IInitializable>
for crate::Zenject::SceneContextRegistryAdderAndRemover {
    fn as_ref(&self) -> &crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+SceneContextRegistryAdderAndRemover")]
impl AsMut<crate::Zenject::IInitializable>
for crate::Zenject::SceneContextRegistryAdderAndRemover {
    fn as_mut(&mut self) -> &mut crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
