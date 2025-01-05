#[cfg(feature = "Zenject+PrefabBindingFinalizer")]
#[repr(C)]
#[derive(Debug)]
pub struct PrefabBindingFinalizer {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::Zenject::ProviderBindingFinalizer>,
    pub _gameObjectBindInfo: quest_hook::libil2cpp::Gc<
        crate::Zenject::GameObjectCreationParameters,
    >,
    pub _prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    pub _providerFactory: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::System::Type>,
        quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabInstantiator>,
        quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
    >,
}
#[cfg(feature = "Zenject+PrefabBindingFinalizer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::PrefabBindingFinalizer => "Zenject"
    ."PrefabBindingFinalizer"
);
#[cfg(feature = "Zenject+PrefabBindingFinalizer")]
impl std::ops::Deref for crate::Zenject::PrefabBindingFinalizer {
    type Target = quest_hook::libil2cpp::Gc<crate::Zenject::ProviderBindingFinalizer>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabBindingFinalizer")]
impl std::ops::DerefMut for crate::Zenject::PrefabBindingFinalizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabBindingFinalizer")]
impl crate::Zenject::PrefabBindingFinalizer {
    pub fn FinalizeBindingConcrete(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        concreteTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinalizeBindingConcrete", (container, concreteTypes))?;
        Ok(__cordl_ret.into())
    }
    pub fn FinalizeBindingSelf(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinalizeBindingSelf", (container))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        providerFactory: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabInstantiator>,
            quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (bindInfo, gameObjectBindInfo, prefab, providerFactory),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn OnFinalizeBinding(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFinalizeBinding", (container))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        providerFactory: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabInstantiator>,
            quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindInfo, gameObjectBindInfo, prefab, providerFactory))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+PrefabBindingFinalizer")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::PrefabBindingFinalizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
