#[cfg(feature = "Zenject+FromBinderNonGeneric")]
#[repr(C)]
#[derive(Debug)]
pub struct FromBinderNonGeneric {
    __cordl_parent: crate::Zenject::FromBinder,
}
#[cfg(feature = "Zenject+FromBinderNonGeneric")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FromBinderNonGeneric => "Zenject"
    ."FromBinderNonGeneric"
);
#[cfg(feature = "Zenject+FromBinderNonGeneric")]
impl std::ops::Deref for crate::Zenject::FromBinderNonGeneric {
    type Target = crate::Zenject::FromBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FromBinderNonGeneric")]
impl std::ops::DerefMut for crate::Zenject::FromBinderNonGeneric {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FromBinderNonGeneric")]
impl crate::Zenject::FromBinderNonGeneric {
    #[cfg(feature = "Zenject+FromBinderNonGeneric+__c__1_2")]
    pub type __c__1_2<
        TConcrete: quest_hook::libil2cpp::Type,
        TFactory: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FromBinderNonGeneric___c__1_2<TConcrete, TFactory>;
    pub fn FromComponentsInChildren_Func_2__cordl_bool0(
        &mut self,
        predicate: *mut crate::System::Func_2<*mut crate::UnityEngine::Component, bool>,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromComponentsInChildren", (predicate, includeInactive))?;
        Ok(__cordl_ret)
    }
    pub fn FromComponentsInChildren__cordl_bool_Func_2__cordl_bool1(
        &mut self,
        excludeSelf: bool,
        predicate: *mut crate::System::Func_2<*mut crate::UnityEngine::Component, bool>,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke(
                "FromComponentsInChildren",
                (excludeSelf, predicate, includeInactive),
            )?;
        Ok(__cordl_ret)
    }
    pub fn FromComponentsInHierarchy(
        &mut self,
        predicate: *mut crate::System::Func_2<*mut crate::UnityEngine::Component, bool>,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromComponentsInHierarchy", (predicate, includeInactive))?;
        Ok(__cordl_ret)
    }
    pub fn FromFactory<TConcrete, TFactory>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactory: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn FromIFactory<TContract>(
        &mut self,
        factoryBindGenerator: *mut crate::System::Action_1<
            *mut crate::Zenject::ConcreteBinderGeneric_1<
                *mut crate::Zenject::IFactory_1<TContract>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromIFactory", (factoryBindGenerator))?;
        Ok(__cordl_ret)
    }
    pub fn FromInstance(
        &mut self,
        instance: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromInstance", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn FromMethod<TConcrete>(
        &mut self,
        method: *mut crate::System::Func_2<*mut crate::Zenject::InjectContext, TConcrete>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromMethod", (method))?;
        Ok(__cordl_ret)
    }
    pub fn FromMethodMultiple<TConcrete>(
        &mut self,
        method: *mut crate::System::Func_2<
            *mut crate::Zenject::InjectContext,
            *mut crate::System::Collections::Generic::IEnumerable_1<TConcrete>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromMethodMultiple", (method))?;
        Ok(__cordl_ret)
    }
    pub fn FromResolveAllGetter_Func_2_0<TObj, TContract>(
        &mut self,
        method: *mut crate::System::Func_2<TObj, TContract>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromResolveAllGetter", (method))?;
        Ok(__cordl_ret)
    }
    pub fn FromResolveAllGetter_Object_Func_2_1<TObj, TContract>(
        &mut self,
        identifier: *mut crate::System::Object,
        method: *mut crate::System::Func_2<TObj, TContract>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromResolveAllGetter", (identifier, method))?;
        Ok(__cordl_ret)
    }
    pub fn FromResolveAllGetter_Object_Func_2_InjectSources2<TObj, TContract>(
        &mut self,
        identifier: *mut crate::System::Object,
        method: *mut crate::System::Func_2<TObj, TContract>,
        source: crate::Zenject::InjectSources,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromResolveAllGetter", (identifier, method, source))?;
        Ok(__cordl_ret)
    }
    pub fn FromResolveGetter_Func_2_0<TObj, TContract>(
        &mut self,
        method: *mut crate::System::Func_2<TObj, TContract>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromResolveGetter", (method))?;
        Ok(__cordl_ret)
    }
    pub fn FromResolveGetter_Object_Func_2_1<TObj, TContract>(
        &mut self,
        identifier: *mut crate::System::Object,
        method: *mut crate::System::Func_2<TObj, TContract>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromResolveGetter", (identifier, method))?;
        Ok(__cordl_ret)
    }
    pub fn FromResolveGetter_Object_Func_2_InjectSources2<TObj, TContract>(
        &mut self,
        identifier: *mut crate::System::Object,
        method: *mut crate::System::Func_2<TObj, TContract>,
        source: crate::Zenject::InjectSources,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromResolveGetter", (identifier, method, source))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bindContainer: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        bindStatement: *mut crate::Zenject::BindStatement,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindContainer, bindInfo, bindStatement))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        bindContainer: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        bindStatement: *mut crate::Zenject::BindStatement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindContainer, bindInfo, bindStatement))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+FromBinderNonGeneric")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::FromBinderNonGeneric {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}