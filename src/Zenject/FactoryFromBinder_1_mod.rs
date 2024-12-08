#[cfg(feature = "Zenject+FactoryFromBinder_1")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryFromBinder_1<TContract: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::Zenject::FactoryFromBinderBase,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "Zenject+FactoryFromBinder_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactoryFromBinder_1 < TContract > =>
    "Zenject"."FactoryFromBinder`1" < TContract >
);
#[cfg(feature = "Zenject+FactoryFromBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::FactoryFromBinder_1<TContract> {
    type Target = crate::Zenject::FactoryFromBinderBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::FactoryFromBinder_1<TContract> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder_1")]
impl<
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::FactoryFromBinder_1<TContract> {
    #[cfg(feature = "Zenject+FactoryFromBinder_1+__c")]
    pub type __c = crate::Zenject::FactoryFromBinder_1___c<TContract>;
    #[cfg(feature = "Zenject+FactoryFromBinder_1+__c__5_1")]
    pub type __c__5_1<TSubFactory: quest_hook::libil2cpp::Type> = crate::Zenject::FactoryFromBinder_1___c__5_1<
        TContract,
        TSubFactory,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder_1+__c__DisplayClass3_0_1")]
    pub type __c__DisplayClass3_0_1<TObj: quest_hook::libil2cpp::Type> = crate::Zenject::FactoryFromBinder_1___c__DisplayClass3_0_1<
        TContract,
        TObj,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder_1+__c__DisplayClass4_0")]
    pub type __c__DisplayClass4_0 = crate::Zenject::FactoryFromBinder_1___c__DisplayClass4_0<
        TContract,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder_1+__c__DisplayClass8_0")]
    pub type __c__DisplayClass8_0 = crate::Zenject::FactoryFromBinder_1___c__DisplayClass8_0<
        TContract,
    >;
    pub fn FromComponentInHierarchy(
        &mut self,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromComponentInHierarchy", (includeInactive))?;
        Ok(__cordl_ret)
    }
    pub fn FromFactory<TSubFactory>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ArgConditionCopyNonLazyBinder,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSubFactory: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn FromMethod(
        &mut self,
        method: *mut crate::System::Func_2<*mut crate::Zenject::DiContainer, TContract>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromMethod", (method))?;
        Ok(__cordl_ret)
    }
    pub fn FromResolveGetter_Func_2_0<TObj>(
        &mut self,
        method: *mut crate::System::Func_2<TObj, TContract>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromResolveGetter", (method))?;
        Ok(__cordl_ret)
    }
    pub fn FromResolveGetter_Object_Func_2_1<TObj>(
        &mut self,
        subIdentifier: *mut crate::System::Object,
        method: *mut crate::System::Func_2<TObj, TContract>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromResolveGetter", (subIdentifier, method))?;
        Ok(__cordl_ret)
    }
    pub fn FromResolveGetter_Object_Func_2_InjectSources2<TObj>(
        &mut self,
        subIdentifier: *mut crate::System::Object,
        method: *mut crate::System::Func_2<TObj, TContract>,
        source: crate::Zenject::InjectSources,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromResolveGetter", (subIdentifier, method, source))?;
        Ok(__cordl_ret)
    }
    pub fn FromSubContainerResolve_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactorySubContainerBinder_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactorySubContainerBinder_1<TContract> = __cordl_object
            .invoke("FromSubContainerResolve", ())?;
        Ok(__cordl_ret)
    }
    pub fn FromSubContainerResolve_Object1(
        &mut self,
        subIdentifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactorySubContainerBinder_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactorySubContainerBinder_1<TContract> = __cordl_object
            .invoke("FromSubContainerResolve", (subIdentifier))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        container: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, bindInfo, factoryBindInfo))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (container, bindInfo, factoryBindInfo))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::FactoryFromBinder_1<TContract> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
