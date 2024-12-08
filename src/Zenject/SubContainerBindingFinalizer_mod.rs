#[cfg(feature = "Zenject+SubContainerBindingFinalizer")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerBindingFinalizer {
    __cordl_parent: crate::Zenject::ProviderBindingFinalizer,
    pub _subIdentifier: *mut crate::System::Object,
    pub _resolveAll: bool,
    pub _creatorFactory: *mut crate::System::Func_2<
        *mut crate::Zenject::DiContainer,
        *mut crate::Zenject::ISubContainerCreator,
    >,
}
#[cfg(feature = "Zenject+SubContainerBindingFinalizer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SubContainerBindingFinalizer =>
    "Zenject"."SubContainerBindingFinalizer"
);
#[cfg(feature = "Zenject+SubContainerBindingFinalizer")]
impl std::ops::Deref for crate::Zenject::SubContainerBindingFinalizer {
    type Target = crate::Zenject::ProviderBindingFinalizer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerBindingFinalizer")]
impl std::ops::DerefMut for crate::Zenject::SubContainerBindingFinalizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerBindingFinalizer")]
impl crate::Zenject::SubContainerBindingFinalizer {
    #[cfg(feature = "Zenject+SubContainerBindingFinalizer+__c__DisplayClass5_0")]
    pub type __c__DisplayClass5_0 = crate::Zenject::SubContainerBindingFinalizer___c__DisplayClass5_0;
    #[cfg(feature = "Zenject+SubContainerBindingFinalizer+__c__DisplayClass5_1")]
    pub type __c__DisplayClass5_1 = crate::Zenject::SubContainerBindingFinalizer___c__DisplayClass5_1;
    #[cfg(feature = "Zenject+SubContainerBindingFinalizer+__c__DisplayClass6_0")]
    pub type __c__DisplayClass6_0 = crate::Zenject::SubContainerBindingFinalizer___c__DisplayClass6_0;
    #[cfg(feature = "Zenject+SubContainerBindingFinalizer+__c__DisplayClass6_1")]
    pub type __c__DisplayClass6_1 = crate::Zenject::SubContainerBindingFinalizer___c__DisplayClass6_1;
    pub fn _ctor(
        &mut self,
        bindInfo: *mut crate::Zenject::BindInfo,
        subIdentifier: *mut crate::System::Object,
        resolveAll: bool,
        creatorFactory: *mut crate::System::Func_2<
            *mut crate::Zenject::DiContainer,
            *mut crate::Zenject::ISubContainerCreator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindInfo, subIdentifier, resolveAll, creatorFactory))?;
        Ok(__cordl_ret)
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
        subIdentifier: *mut crate::System::Object,
        resolveAll: bool,
        creatorFactory: *mut crate::System::Func_2<
            *mut crate::Zenject::DiContainer,
            *mut crate::Zenject::ISubContainerCreator,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (bindInfo, subIdentifier, resolveAll, creatorFactory),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+SubContainerBindingFinalizer")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::SubContainerBindingFinalizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
