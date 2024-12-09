#[cfg(feature = "Zenject+FactoryFromBinder_2")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryFromBinder_2<
    TParam1: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Zenject::FactoryFromBinderBase,
    __cordl_phantom_TParam1: std::marker::PhantomData<TParam1>,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "Zenject+FactoryFromBinder_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactoryFromBinder_2 < TParam1,
    TContract > => "Zenject"."FactoryFromBinder`2" < TParam1, TContract >
);
#[cfg(feature = "Zenject+FactoryFromBinder_2")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::Zenject::FactoryFromBinder_2<TParam1, TContract> {
    type Target = crate::Zenject::FactoryFromBinderBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder_2")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::Zenject::FactoryFromBinder_2<TParam1, TContract> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder_2")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::FactoryFromBinder_2<TParam1, TContract> {
    #[cfg(feature = "Zenject+FactoryFromBinder_2+__c__2_1")]
    pub type __c__2_1<TSubFactory: quest_hook::libil2cpp::Type> = crate::Zenject::FactoryFromBinder_2___c__2_1<
        TParam1,
        TContract,
        TSubFactory,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder_2+__c__DisplayClass1_0")]
    pub type __c__DisplayClass1_0 = crate::Zenject::FactoryFromBinder_2___c__DisplayClass1_0<
        TParam1,
        TContract,
    >;
    pub fn FromFactory<TSubFactory>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSubFactory: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn FromMethod(
        &mut self,
        method: *mut crate::System::Func_3<
            *mut crate::Zenject::DiContainer,
            TParam1,
            TContract,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
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
    pub fn FromSubContainerResolve_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactorySubContainerBinder_2<TParam1, TContract>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactorySubContainerBinder_2<
            TParam1,
            TContract,
        > = __cordl_object.invoke("FromSubContainerResolve", ())?;
        Ok(__cordl_ret)
    }
    pub fn FromSubContainerResolve_Il2CppObject1(
        &mut self,
        subIdentifier: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactorySubContainerBinder_2<TParam1, TContract>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactorySubContainerBinder_2<
            TParam1,
            TContract,
        > = __cordl_object.invoke("FromSubContainerResolve", (subIdentifier))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        container: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
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
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
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
#[cfg(feature = "Zenject+FactoryFromBinder_2")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Zenject::FactoryFromBinder_2<TParam1, TContract> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
