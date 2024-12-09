#[cfg(feature = "Zenject+FactorySubContainerBinder_4")]
#[repr(C)]
#[derive(Debug)]
pub struct FactorySubContainerBinder_4<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Zenject::FactorySubContainerBinderWithParams_1<TContract>,
    __cordl_phantom_TParam1: std::marker::PhantomData<TParam1>,
    __cordl_phantom_TParam2: std::marker::PhantomData<TParam2>,
    __cordl_phantom_TParam3: std::marker::PhantomData<TParam3>,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "Zenject+FactorySubContainerBinder_4")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactorySubContainerBinder_4 < TParam1,
    TParam2, TParam3, TContract > => "Zenject"."FactorySubContainerBinder`4" < TParam1,
    TParam2, TParam3, TContract >
);
#[cfg(feature = "Zenject+FactorySubContainerBinder_4")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Zenject::FactorySubContainerBinder_4<TParam1, TParam2, TParam3, TContract> {
    type Target = crate::Zenject::FactorySubContainerBinderWithParams_1<TContract>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactorySubContainerBinder_4")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Zenject::FactorySubContainerBinder_4<TParam1, TParam2, TParam3, TContract> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactorySubContainerBinder_4")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::FactorySubContainerBinder_4<TParam1, TParam2, TParam3, TContract> {
    #[cfg(feature = "Zenject+FactorySubContainerBinder_4+__c__DisplayClass1_0")]
    pub type __c__DisplayClass1_0 = crate::Zenject::FactorySubContainerBinder_4___c__DisplayClass1_0<
        TParam1,
        TParam2,
        TParam3,
        TContract,
    >;
    #[cfg(feature = "Zenject+FactorySubContainerBinder_4+__c__DisplayClass2_0")]
    pub type __c__DisplayClass2_0 = crate::Zenject::FactorySubContainerBinder_4___c__DisplayClass2_0<
        TParam1,
        TParam2,
        TParam3,
        TContract,
    >;
    #[cfg(feature = "Zenject+FactorySubContainerBinder_4+__c__DisplayClass3_0")]
    pub type __c__DisplayClass3_0 = crate::Zenject::FactorySubContainerBinder_4___c__DisplayClass3_0<
        TParam1,
        TParam2,
        TParam3,
        TContract,
    >;
    #[cfg(feature = "Zenject+FactorySubContainerBinder_4+__c__DisplayClass4_0")]
    pub type __c__DisplayClass4_0 = crate::Zenject::FactorySubContainerBinder_4___c__DisplayClass4_0<
        TParam1,
        TParam2,
        TParam3,
        TContract,
    >;
    pub fn ByMethod(
        &mut self,
        installerMethod: *mut crate::System::Action_4<
            *mut crate::Zenject::DiContainer,
            TParam1,
            TParam2,
            TParam3,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByMethod", (installerMethod))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewGameObjectMethod(
        &mut self,
        installerMethod: *mut crate::System::Action_4<
            *mut crate::Zenject::DiContainer,
            TParam1,
            TParam2,
            TParam3,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewGameObjectMethod", (installerMethod))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewPrefabMethod(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        installerMethod: *mut crate::System::Action_4<
            *mut crate::Zenject::DiContainer,
            TParam1,
            TParam2,
            TParam3,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewPrefabMethod", (prefab, installerMethod))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewPrefabResourceMethod(
        &mut self,
        resourcePath: *mut crate::System::String,
        installerMethod: *mut crate::System::Action_4<
            *mut crate::Zenject::DiContainer,
            TParam1,
            TParam2,
            TParam3,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewPrefabResourceMethod", (resourcePath, installerMethod))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bindContainer: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
        subIdentifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (bindContainer, bindInfo, factoryBindInfo, subIdentifier),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        bindContainer: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
        subIdentifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindContainer, bindInfo, factoryBindInfo, subIdentifier))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+FactorySubContainerBinder_4")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Zenject::FactorySubContainerBinder_4<TParam1, TParam2, TParam3, TContract> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
