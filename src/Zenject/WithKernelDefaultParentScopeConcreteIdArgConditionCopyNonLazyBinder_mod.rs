#[cfg(
    feature = "Zenject+WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder"
)]
#[repr(C)]
#[derive(Debug)]
pub struct WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder {
    __cordl_parent: crate::Zenject::DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder,
}
#[cfg(
    feature = "Zenject+WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder =>
    "Zenject"."WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder"
);
#[cfg(
    feature = "Zenject+WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder"
)]
impl std::ops::Deref
for crate::Zenject::WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder {
    type Target = crate::Zenject::DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Zenject+WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder"
)]
impl std::ops::DerefMut
for crate::Zenject::WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Zenject+WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder"
)]
impl crate::Zenject::WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder {
    pub fn New(
        subContainerBindInfo: *mut crate::Zenject::SubContainerCreatorBindInfo,
        bindInfo: *mut crate::Zenject::BindInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (subContainerBindInfo, bindInfo))?;
        Ok(__cordl_object)
    }
    pub fn WithKernel_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("WithKernel", ())?;
        Ok(__cordl_ret)
    }
    pub fn WithKernel_1<TKernel>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TKernel: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("WithKernel", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        subContainerBindInfo: *mut crate::Zenject::SubContainerCreatorBindInfo,
        bindInfo: *mut crate::Zenject::BindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (subContainerBindInfo, bindInfo))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "Zenject+WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
