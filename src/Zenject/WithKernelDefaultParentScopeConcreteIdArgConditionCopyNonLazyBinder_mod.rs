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
unsafe impl quest_hook::libil2cpp::Type
for crate::Zenject::WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder";
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
            crate::Zenject::DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder,
                >,
                0usize,
            >("WithKernel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WithKernel", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn WithKernel_1<TKernel>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TKernel: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder,
                >,
                0usize,
            >("WithKernel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WithKernel", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        subContainerBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::SubContainerCreatorBindInfo,
        >,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Zenject::SubContainerCreatorBindInfo,
                    >,
                    quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (subContainerBindInfo, bindInfo))
        };
        Ok(__cordl_ret.into())
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
