#[cfg(feature = "Zenject+FactorySubContainerBinderWithParams_1")]
#[repr(C)]
#[derive(Debug)]
pub struct FactorySubContainerBinderWithParams_1<
    TContract: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Zenject::FactorySubContainerBinderBase_1<TContract>,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "Zenject+FactorySubContainerBinderWithParams_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactorySubContainerBinderWithParams_1 <
    TContract > => "Zenject"."FactorySubContainerBinderWithParams`1" < TContract >
);
#[cfg(feature = "Zenject+FactorySubContainerBinderWithParams_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::FactorySubContainerBinderWithParams_1<TContract> {
    type Target = crate::Zenject::FactorySubContainerBinderBase_1<TContract>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactorySubContainerBinderWithParams_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::FactorySubContainerBinderWithParams_1<TContract> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactorySubContainerBinderWithParams_1")]
impl<
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::FactorySubContainerBinderWithParams_1<TContract> {
    #[cfg(
        feature = "Zenject+FactorySubContainerBinderWithParams_1+__c__DisplayClass8_0"
    )]
    pub type __c__DisplayClass8_0 = crate::Zenject::FactorySubContainerBinderWithParams_1___c__DisplayClass8_0<
        TContract,
    >;
    #[cfg(
        feature = "Zenject+FactorySubContainerBinderWithParams_1+__c__DisplayClass4_0"
    )]
    pub type __c__DisplayClass4_0 = crate::Zenject::FactorySubContainerBinderWithParams_1___c__DisplayClass4_0<
        TContract,
    >;
    pub fn ByNewPrefab_Type_Object0(
        &mut self,
        installerType: *mut crate::System::Type,
        prefab: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewPrefab", (installerType, prefab))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewPrefab_Object1<TInstaller>(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TInstaller: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewPrefab", (prefab))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewContextPrefab_Object0<TInstaller>(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TInstaller: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewContextPrefab", (prefab))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewContextPrefab_Type_Object1(
        &mut self,
        installerType: *mut crate::System::Type,
        prefab: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewContextPrefab", (installerType, prefab))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewContextPrefabResource_String0<TInstaller>(
        &mut self,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TInstaller: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewContextPrefabResource", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewContextPrefabResource_Type_String1(
        &mut self,
        installerType: *mut crate::System::Type,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewContextPrefabResource", (installerType, resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bindContainer: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
        subIdentifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
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
    pub fn ByNewPrefabResource_String0<TInstaller>(
        &mut self,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TInstaller: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewPrefabResource", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewPrefabResource_Type_String1(
        &mut self,
        installerType: *mut crate::System::Type,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewPrefabResource", (installerType, resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bindContainer: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
        subIdentifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (bindContainer, bindInfo, factoryBindInfo, subIdentifier),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+FactorySubContainerBinderWithParams_1")]
impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::FactorySubContainerBinderWithParams_1<TContract> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
