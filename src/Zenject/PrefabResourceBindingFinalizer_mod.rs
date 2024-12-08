#[cfg(feature = "Zenject+PrefabResourceBindingFinalizer")]
#[repr(C)]
#[derive(Debug)]
pub struct PrefabResourceBindingFinalizer {
    __cordl_parent: crate::Zenject::ProviderBindingFinalizer,
    pub _gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
    pub _resourcePath: *mut crate::System::String,
    pub _providerFactory: *mut crate::System::Func_3<
        *mut crate::System::Type,
        *mut crate::Zenject::IPrefabInstantiator,
        *mut crate::Zenject::IProvider,
    >,
}
#[cfg(feature = "Zenject+PrefabResourceBindingFinalizer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::PrefabResourceBindingFinalizer =>
    "Zenject"."PrefabResourceBindingFinalizer"
);
#[cfg(feature = "Zenject+PrefabResourceBindingFinalizer")]
impl std::ops::Deref for crate::Zenject::PrefabResourceBindingFinalizer {
    type Target = crate::Zenject::ProviderBindingFinalizer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabResourceBindingFinalizer")]
impl std::ops::DerefMut for crate::Zenject::PrefabResourceBindingFinalizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabResourceBindingFinalizer")]
impl crate::Zenject::PrefabResourceBindingFinalizer {
    #[cfg(feature = "Zenject+PrefabResourceBindingFinalizer+__c__DisplayClass6_0")]
    pub type __c__DisplayClass6_0 = crate::Zenject::PrefabResourceBindingFinalizer___c__DisplayClass6_0;
    #[cfg(feature = "Zenject+PrefabResourceBindingFinalizer+__c__DisplayClass5_1")]
    pub type __c__DisplayClass5_1 = crate::Zenject::PrefabResourceBindingFinalizer___c__DisplayClass5_1;
    #[cfg(feature = "Zenject+PrefabResourceBindingFinalizer+__c__DisplayClass5_0")]
    pub type __c__DisplayClass5_0 = crate::Zenject::PrefabResourceBindingFinalizer___c__DisplayClass5_0;
    #[cfg(feature = "Zenject+PrefabResourceBindingFinalizer+__c__DisplayClass6_1")]
    pub type __c__DisplayClass6_1 = crate::Zenject::PrefabResourceBindingFinalizer___c__DisplayClass6_1;
    pub fn FinalizeBindingConcrete(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        concreteTypes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Type,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinalizeBindingConcrete", (container, concreteTypes))?;
        Ok(__cordl_ret)
    }
    pub fn FinalizeBindingSelf(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinalizeBindingSelf", (container))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bindInfo: *mut crate::Zenject::BindInfo,
        gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
        resourcePath: *mut crate::System::String,
        providerFactory: *mut crate::System::Func_3<
            *mut crate::System::Type,
            *mut crate::Zenject::IPrefabInstantiator,
            *mut crate::Zenject::IProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (bindInfo, gameObjectBindInfo, resourcePath, providerFactory),
            )?;
        Ok(__cordl_object)
    }
    pub fn OnFinalizeBinding(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFinalizeBinding", (container))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bindInfo: *mut crate::Zenject::BindInfo,
        gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
        resourcePath: *mut crate::System::String,
        providerFactory: *mut crate::System::Func_3<
            *mut crate::System::Type,
            *mut crate::Zenject::IPrefabInstantiator,
            *mut crate::Zenject::IProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (bindInfo, gameObjectBindInfo, resourcePath, providerFactory),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+PrefabResourceBindingFinalizer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::PrefabResourceBindingFinalizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
