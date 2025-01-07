#[cfg(feature = "Zenject+WithKernelScopeConcreteIdArgConditionCopyNonLazyBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct WithKernelScopeConcreteIdArgConditionCopyNonLazyBinder {
    __cordl_parent: crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    pub _subContainerBindInfo: quest_hook::libil2cpp::Gc<
        crate::Zenject::SubContainerCreatorBindInfo,
    >,
}
#[cfg(feature = "Zenject+WithKernelScopeConcreteIdArgConditionCopyNonLazyBinder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Zenject::WithKernelScopeConcreteIdArgConditionCopyNonLazyBinder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "WithKernelScopeConcreteIdArgConditionCopyNonLazyBinder";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Zenject+WithKernelScopeConcreteIdArgConditionCopyNonLazyBinder")]
impl std::ops::Deref
for crate::Zenject::WithKernelScopeConcreteIdArgConditionCopyNonLazyBinder {
    type Target = crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+WithKernelScopeConcreteIdArgConditionCopyNonLazyBinder")]
impl std::ops::DerefMut
for crate::Zenject::WithKernelScopeConcreteIdArgConditionCopyNonLazyBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+WithKernelScopeConcreteIdArgConditionCopyNonLazyBinder")]
impl crate::Zenject::WithKernelScopeConcreteIdArgConditionCopyNonLazyBinder {
    pub fn New(
        subContainerBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::SubContainerCreatorBindInfo,
        >,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (subContainerBindInfo, bindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn WithKernel_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("WithKernel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WithKernel_1<TKernel>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TKernel: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("WithKernel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        subContainerBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::SubContainerCreatorBindInfo,
        >,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (subContainerBindInfo, bindInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+WithKernelScopeConcreteIdArgConditionCopyNonLazyBinder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::WithKernelScopeConcreteIdArgConditionCopyNonLazyBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
