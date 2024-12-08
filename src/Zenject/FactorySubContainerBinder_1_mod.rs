#[cfg(feature = "Zenject+FactorySubContainerBinder_1")]
#[repr(C)]
#[derive(Debug)]
pub struct FactorySubContainerBinder_1<TContract: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::Zenject::FactorySubContainerBinderBase_1<TContract>,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "Zenject+FactorySubContainerBinder_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactorySubContainerBinder_1 < TContract
    > => "Zenject"."FactorySubContainerBinder`1" < TContract >
);
#[cfg(feature = "Zenject+FactorySubContainerBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::FactorySubContainerBinder_1<TContract> {
    type Target = crate::Zenject::FactorySubContainerBinderBase_1<TContract>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactorySubContainerBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::FactorySubContainerBinder_1<TContract> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactorySubContainerBinder_1")]
impl<
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::FactorySubContainerBinder_1<TContract> {
    #[cfg(feature = "Zenject+FactorySubContainerBinder_1+__c__DisplayClass3_0")]
    pub type __c__DisplayClass3_0 = crate::Zenject::FactorySubContainerBinder_1___c__DisplayClass3_0<
        TContract,
    >;
    #[cfg(feature = "Zenject+FactorySubContainerBinder_1+__c__DisplayClass4_0")]
    pub type __c__DisplayClass4_0 = crate::Zenject::FactorySubContainerBinder_1___c__DisplayClass4_0<
        TContract,
    >;
    #[cfg(feature = "Zenject+FactorySubContainerBinder_1+__c__DisplayClass6_0")]
    pub type __c__DisplayClass6_0 = crate::Zenject::FactorySubContainerBinder_1___c__DisplayClass6_0<
        TContract,
    >;
    #[cfg(feature = "Zenject+FactorySubContainerBinder_1+__c__DisplayClass2_0")]
    pub type __c__DisplayClass2_0 = crate::Zenject::FactorySubContainerBinder_1___c__DisplayClass2_0<
        TContract,
    >;
    #[cfg(feature = "Zenject+FactorySubContainerBinder_1+__c__DisplayClass1_0")]
    pub type __c__DisplayClass1_0 = crate::Zenject::FactorySubContainerBinder_1___c__DisplayClass1_0<
        TContract,
    >;
    #[cfg(feature = "Zenject+FactorySubContainerBinder_1+__c__DisplayClass8_0")]
    pub type __c__DisplayClass8_0 = crate::Zenject::FactorySubContainerBinder_1___c__DisplayClass8_0<
        TContract,
    >;
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
    pub fn ByNewPrefabResource(
        &mut self,
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
            .invoke("ByNewPrefabResource", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewContextPrefabResource(
        &mut self,
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
            .invoke("ByNewContextPrefabResource", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewPrefabResourceMethod(
        &mut self,
        resourcePath: *mut crate::System::String,
        installerMethod: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
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
            .invoke("ByNewPrefabResourceMethod", (resourcePath, installerMethod))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewPrefab(
        &mut self,
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
            .invoke("ByNewPrefab", (prefab))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewContextPrefab(
        &mut self,
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
            .invoke("ByNewContextPrefab", (prefab))?;
        Ok(__cordl_ret)
    }
    pub fn ByMethod(
        &mut self,
        installerMethod: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
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
        installerMethod: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
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
            .invoke("ByNewGameObjectMethod", (installerMethod))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewPrefabMethod(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        installerMethod: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
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
            .invoke("ByNewPrefabMethod", (prefab, installerMethod))?;
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
#[cfg(feature = "Zenject+FactorySubContainerBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::FactorySubContainerBinder_1<TContract> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
