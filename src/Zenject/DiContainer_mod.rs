#[cfg(feature = "Zenject+DiContainer")]
#[repr(C)]
#[derive(Debug)]
pub struct DiContainer {
    __cordl_parent: crate::System::Object,
    pub _decorators: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Type,
        *mut crate::Zenject::Internal::IDecoratorProvider,
    >,
    pub _providers: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::Zenject::BindingId,
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::DiContainer_ProviderInfo,
        >,
    >,
    pub _containerLookups: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::Zenject::DiContainer>,
    >,
    pub _resolvesInProgress: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::Zenject::Internal::LookupId,
    >,
    pub _resolvesTwiceInProgress: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::Zenject::Internal::LookupId,
    >,
    pub _lazyInjector: *mut crate::Zenject::LazyInstanceInjector,
    pub _singletonMarkRegistry: *mut crate::Zenject::Internal::SingletonMarkRegistry,
    pub _currentBindings: *mut crate::System::Collections::Generic::Queue_1<
        *mut crate::Zenject::BindStatement,
    >,
    pub _childBindings: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::BindStatement,
    >,
    pub _validatedTypes: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::Type,
    >,
    pub _validationQueue: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::IValidatable,
    >,
    pub _contextTransform: *mut crate::UnityEngine::Transform,
    pub _hasLookedUpContextTransform: bool,
    pub _inheritedDefaultParent: *mut crate::UnityEngine::Transform,
    pub _explicitDefaultParent: *mut crate::UnityEngine::Transform,
    pub _hasExplicitDefaultParent: bool,
    pub _settings: *mut crate::Zenject::ZenjectSettings,
    pub _hasResolvedRoots: bool,
    pub _isFinalizingBinding: bool,
    pub _isValidating: bool,
    pub _isInstalling: bool,
    pub _AssertOnNewGameObjects_k__BackingField: bool,
}
#[cfg(feature = "Zenject+DiContainer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::DiContainer => "Zenject"."DiContainer"
);
#[cfg(feature = "Zenject+DiContainer")]
impl std::ops::Deref for crate::Zenject::DiContainer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DiContainer")]
impl std::ops::DerefMut for crate::Zenject::DiContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DiContainer")]
impl crate::Zenject::DiContainer {
    #[cfg(feature = "Zenject+DiContainer+__c")]
    pub type __c = crate::Zenject::DiContainer___c;
    #[cfg(feature = "Zenject+DiContainer+_GetDependencyContracts_d__96")]
    pub type _GetDependencyContracts_d__96 = crate::Zenject::DiContainer__GetDependencyContracts_d__96;
    #[cfg(feature = "Zenject+DiContainer+__c__DisplayClass178_0")]
    pub type __c__DisplayClass178_0 = crate::Zenject::DiContainer___c__DisplayClass178_0;
    #[cfg(feature = "Zenject+DiContainer+__c__DisplayClass203_0_1")]
    pub type __c__DisplayClass203_0_1<TContract: quest_hook::libil2cpp::Type> = crate::Zenject::DiContainer___c__DisplayClass203_0_1<
        TContract,
    >;
    #[cfg(feature = "Zenject+DiContainer+__c__DisplayClass86_0")]
    pub type __c__DisplayClass86_0 = crate::Zenject::DiContainer___c__DisplayClass86_0;
    #[cfg(feature = "Zenject+DiContainer+ProviderInfo")]
    pub type ProviderInfo = crate::Zenject::DiContainer_ProviderInfo;
    pub fn InjectExplicit_List_1_0(
        &mut self,
        injectable: *mut crate::System::Object,
        extraArgs: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InjectExplicit", (injectable, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InjectExplicit_Type_List_1_InjectContext_Object1(
        &mut self,
        injectable: *mut crate::System::Object,
        injectableType: *mut crate::System::Type,
        extraArgs: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        context: *mut crate::Zenject::InjectContext,
        concreteIdentifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InjectExplicit",
                (injectable, injectableType, extraArgs, context, concreteIdentifier),
            )?;
        Ok(__cordl_ret)
    }
    pub fn BindIFactory_0<TContract>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_1<TContract> = __cordl_object
            .invoke("BindIFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindIFactory_1<TParam1, TContract>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_2<TParam1, TContract>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_2<
            TParam1,
            TContract,
        > = __cordl_object.invoke("BindIFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindIFactory_2<TParam1, TParam2, TContract>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_3<TParam1, TParam2, TContract>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_3<
            TParam1,
            TParam2,
            TContract,
        > = __cordl_object.invoke("BindIFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindIFactory_3<TParam1, TParam2, TParam3, TContract>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_4<
            TParam1,
            TParam2,
            TParam3,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_4<
            TParam1,
            TParam2,
            TParam3,
            TContract,
        > = __cordl_object.invoke("BindIFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindIFactory_4<TParam1, TParam2, TParam3, TParam4, TContract>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_5<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_5<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TContract,
        > = __cordl_object.invoke("BindIFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindIFactory_5<TParam1, TParam2, TParam3, TParam4, TParam5, TContract>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_6<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_6<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TContract,
        > = __cordl_object.invoke("BindIFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindIFactory_6<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TContract,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_7<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_7<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TContract,
        > = __cordl_object.invoke("BindIFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindIFactory_7<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TParam8,
        TParam9,
        TParam10,
        TContract,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_11<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TParam7,
            TParam8,
            TParam9,
            TParam10,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam7: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam8: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam9: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam10: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_11<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TParam7,
            TParam8,
            TParam9,
            TParam10,
            TContract,
        > = __cordl_object.invoke("BindIFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn LazyInject<T>(&mut self, instance: T) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("LazyInject", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn set_DefaultParent(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DefaultParent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn BindInternal(
        &mut self,
        bindInfo: *mut crate::Zenject::BindInfo,
        bindingFinalizer: *mut crate::Zenject::BindStatement,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConcreteIdBinderNonGeneric> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConcreteIdBinderNonGeneric = __cordl_object
            .invoke("BindInternal", (bindInfo, bindingFinalizer))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateInternal(
        &mut self,
        concreteType: *mut crate::System::Type,
        autoInject: bool,
        extraArgs: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        context: *mut crate::Zenject::InjectContext,
        concreteIdentifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InstantiateInternal",
                (concreteType, autoInject, extraArgs, context, concreteIdentifier),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateScriptableObjectResourceExplicit(
        &mut self,
        scriptableObjectType: *mut crate::System::Type,
        resourcePath: *mut crate::System::String,
        extraArgs: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InstantiateScriptableObjectResourceExplicit",
                (scriptableObjectType, resourcePath, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ResolveIdAll_Object0<TContract>(
        &mut self,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<TContract> = __cordl_object
            .invoke("ResolveIdAll", (identifier))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveIdAll_Type_Object1(
        &mut self,
        contractType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("ResolveIdAll", (contractType, identifier))?;
        Ok(__cordl_ret)
    }
    pub fn get_ParentContainers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::Zenject::DiContainer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Zenject::DiContainer,
        > = __cordl_object.invoke("get_ParentContainers", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IEnumerable_1__cordl_bool0(
        &mut self,
        parentContainersEnumerable: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::DiContainer,
        >,
        isValidating: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parentContainersEnumerable, isValidating))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        isValidating: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (isValidating))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DiContainer__cordl_bool3(
        &mut self,
        parentContainer: *mut crate::Zenject::DiContainer,
        isValidating: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parentContainer, isValidating))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DiContainer4(
        &mut self,
        parentContainer: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parentContainer))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IEnumerable_1_5(
        &mut self,
        parentContainers: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::DiContainer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parentContainers))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveRoots(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResolveRoots", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindNoFlush<TContract>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ConcreteIdBinderGeneric_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConcreteIdBinderGeneric_1<TContract> = __cordl_object
            .invoke("BindNoFlush", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDependencyContracts_0<TContract>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Type,
        > = __cordl_object.invoke("GetDependencyContracts", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDependencyContracts_Type1(
        &mut self,
        contract: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Type,
        > = __cordl_object.invoke("GetDependencyContracts", (contract))?;
        Ok(__cordl_ret)
    }
    pub fn BindMemoryPoolCustomInterfaceInternal<
        TItemContract,
        TPoolConcrete,
        TPoolContract,
    >(
        &mut self,
        includeConcreteType: bool,
        statement: *mut crate::Zenject::BindStatement,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::MemoryPoolIdInitialSizeMaxSizeBinder_1<TItemContract>,
    >
    where
        TItemContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TPoolConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TPoolContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::MemoryPoolIdInitialSizeMaxSizeBinder_1<
            TItemContract,
        > = __cordl_object
            .invoke(
                "BindMemoryPoolCustomInterfaceInternal",
                (includeConcreteType, statement),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InjectGameObjectForComponentExplicit(
        &mut self,
        gameObject: *mut crate::UnityEngine::GameObject,
        componentType: *mut crate::System::Type,
        extraArgs: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        context: *mut crate::Zenject::InjectContext,
        concreteIdentifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke(
                "InjectGameObjectForComponentExplicit",
                (gameObject, componentType, extraArgs, context, concreteIdentifier),
            )?;
        Ok(__cordl_ret)
    }
    pub fn TryResolve_0<TContract>(&mut self) -> quest_hook::libil2cpp::Result<TContract>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TContract = __cordl_object.invoke("TryResolve", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryResolve_Type1(
        &mut self,
        contractType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("TryResolve", (contractType))?;
        Ok(__cordl_ret)
    }
    pub fn StartBinding(
        &mut self,
        flush: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::BindStatement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::BindStatement = __cordl_object
            .invoke("StartBinding", (flush))?;
        Ok(__cordl_ret)
    }
    pub fn BindInterfacesAndSelfTo_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::FromBinderNonGeneric>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FromBinderNonGeneric = __cordl_object
            .invoke("BindInterfacesAndSelfTo", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindInterfacesAndSelfTo_Type1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::FromBinderNonGeneric> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FromBinderNonGeneric = __cordl_object
            .invoke("BindInterfacesAndSelfTo", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetLocalProviders(
        &mut self,
        bindingId: crate::Zenject::BindingId,
        buffer: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::DiContainer_ProviderInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetLocalProviders", (bindingId, buffer))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResourceForComponentExplicit_GameObjectCreationParameters0(
        &mut self,
        componentType: *mut crate::System::Type,
        resourcePath: *mut crate::System::String,
        extraArgs: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        creationInfo: *mut crate::Zenject::GameObjectCreationParameters,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InstantiatePrefabResourceForComponentExplicit",
                (componentType, resourcePath, extraArgs, creationInfo),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResourceForComponentExplicit_InjectContext_Object_GameObjectCreationParameters1(
        &mut self,
        componentType: *mut crate::System::Type,
        resourcePath: *mut crate::System::String,
        extraArgs: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        context: *mut crate::Zenject::InjectContext,
        concreteIdentifier: *mut crate::System::Object,
        creationInfo: *mut crate::Zenject::GameObjectCreationParameters,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InstantiatePrefabResourceForComponentExplicit",
                (
                    componentType,
                    resourcePath,
                    extraArgs,
                    context,
                    concreteIdentifier,
                    creationInfo,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetPrefabAsGameObject(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("GetPrefabAsGameObject", (prefab))?;
        Ok(__cordl_ret)
    }
    pub fn BindFixedTickableExecutionOrder_i32_0<T>(
        &mut self,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("BindFixedTickableExecutionOrder", (order))?;
        Ok(__cordl_ret)
    }
    pub fn BindFixedTickableExecutionOrder_Type_i32_1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("BindFixedTickableExecutionOrder", (_cordl_type, order))?;
        Ok(__cordl_ret)
    }
    pub fn SafeGetInstances(
        &mut self,
        providerInfo: *mut crate::Zenject::DiContainer_ProviderInfo,
        context: *mut crate::Zenject::InjectContext,
        instances: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SafeGetInstances", (providerInfo, context, instances))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResource_String0(
        &mut self,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("InstantiatePrefabResource", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResource_Transform1(
        &mut self,
        resourcePath: *mut crate::System::String,
        parentTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("InstantiatePrefabResource", (resourcePath, parentTransform))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResource_Vector3_Quaternion_Transform2(
        &mut self,
        resourcePath: *mut crate::System::String,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke(
                "InstantiatePrefabResource",
                (resourcePath, position, rotation, parentTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResource_GameObjectCreationParameters3(
        &mut self,
        resourcePath: *mut crate::System::String,
        creationInfo: *mut crate::Zenject::GameObjectCreationParameters,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("InstantiatePrefabResource", (resourcePath, creationInfo))?;
        Ok(__cordl_ret)
    }
    pub fn UnbindAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnbindAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetProvidersForContract(
        &mut self,
        bindingId: crate::Zenject::BindingId,
        sourceType: crate::Zenject::InjectSources,
        buffer: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::DiContainer_ProviderInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetProvidersForContract", (bindingId, sourceType, buffer))?;
        Ok(__cordl_ret)
    }
    pub fn Instantiate_0<T>(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("Instantiate", ())?;
        Ok(__cordl_ret)
    }
    pub fn Instantiate_IEnumerable_1_1<T>(
        &mut self,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("Instantiate", (extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn Instantiate_Type2(
        &mut self,
        concreteType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Instantiate", (concreteType))?;
        Ok(__cordl_ret)
    }
    pub fn Instantiate_Type_IEnumerable_1_3(
        &mut self,
        concreteType: *mut crate::System::Type,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Instantiate", (concreteType, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InjectGameObjectForComponent_GameObject0<T>(
        &mut self,
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InjectGameObjectForComponent", (gameObject))?;
        Ok(__cordl_ret)
    }
    pub fn InjectGameObjectForComponent_IEnumerable_1_1<T>(
        &mut self,
        gameObject: *mut crate::UnityEngine::GameObject,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InjectGameObjectForComponent", (gameObject, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InjectGameObjectForComponent_Type_IEnumerable_1_2(
        &mut self,
        gameObject: *mut crate::UnityEngine::GameObject,
        componentType: *mut crate::System::Type,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InjectGameObjectForComponent",
                (gameObject, componentType, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn BindMemoryPool_0<TItemContract>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::MemoryPoolIdInitialSizeMaxSizeBinder_1<TItemContract>,
    >
    where
        TItemContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::MemoryPoolIdInitialSizeMaxSizeBinder_1<
            TItemContract,
        > = __cordl_object.invoke("BindMemoryPool", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindMemoryPool_1<TItemContract, TPool>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::MemoryPoolIdInitialSizeMaxSizeBinder_1<TItemContract>,
    >
    where
        TItemContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TPool: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::MemoryPoolIdInitialSizeMaxSizeBinder_1<
            TItemContract,
        > = __cordl_object.invoke("BindMemoryPool", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateExplicit_List_1_0<T>(
        &mut self,
        extraArgs: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("InstantiateExplicit", (extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateExplicit_Type_List_1_1(
        &mut self,
        concreteType: *mut crate::System::Type,
        extraArgs: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("InstantiateExplicit", (concreteType, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateExplicit_Type__cordl_bool_List_1_InjectContext_Object2(
        &mut self,
        concreteType: *mut crate::System::Type,
        autoInject: bool,
        extraArgs: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        context: *mut crate::Zenject::InjectContext,
        concreteIdentifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InstantiateExplicit",
                (concreteType, autoInject, extraArgs, context, concreteIdentifier),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponentExplicit_Type_Object_List_1_0(
        &mut self,
        componentType: *mut crate::System::Type,
        prefab: *mut crate::UnityEngine::Object,
        extraArgs: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InstantiatePrefabForComponentExplicit",
                (componentType, prefab, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponentExplicit_GameObjectCreationParameters1(
        &mut self,
        componentType: *mut crate::System::Type,
        prefab: *mut crate::UnityEngine::Object,
        extraArgs: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InstantiatePrefabForComponentExplicit",
                (componentType, prefab, extraArgs, gameObjectBindInfo),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponentExplicit_InjectContext_Object_GameObjectCreationParameters2(
        &mut self,
        componentType: *mut crate::System::Type,
        prefab: *mut crate::UnityEngine::Object,
        extraArgs: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        context: *mut crate::Zenject::InjectContext,
        concreteIdentifier: *mut crate::System::Object,
        gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InstantiatePrefabForComponentExplicit",
                (
                    componentType,
                    prefab,
                    extraArgs,
                    context,
                    concreteIdentifier,
                    gameObjectBindInfo,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn FlattenInheritanceChain(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::DiContainer,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::DiContainer,
        > = __cordl_object.invoke("FlattenInheritanceChain", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_AssertOnNewGameObjects(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AssertOnNewGameObjects", (value))?;
        Ok(__cordl_ret)
    }
    pub fn CreateTransformGroup(
        &mut self,
        groupName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("CreateTransformGroup", (groupName))?;
        Ok(__cordl_ret)
    }
    pub fn get_ChecksForCircularDependencies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ChecksForCircularDependencies", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasBinding_0<TContract>(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasBinding", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasBinding_Type1(
        &mut self,
        contractType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasBinding", (contractType))?;
        Ok(__cordl_ret)
    }
    pub fn HasBinding_InjectContext2(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasBinding", (context))?;
        Ok(__cordl_ret)
    }
    pub fn Resolve_BindingId0(
        &mut self,
        id: crate::Zenject::BindingId,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Resolve", (id))?;
        Ok(__cordl_ret)
    }
    pub fn Resolve_InjectContext1(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Resolve", (context))?;
        Ok(__cordl_ret)
    }
    pub fn Resolve_2<TContract>(&mut self) -> quest_hook::libil2cpp::Result<TContract>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TContract = __cordl_object.invoke("Resolve", ())?;
        Ok(__cordl_ret)
    }
    pub fn Resolve_Type3(
        &mut self,
        contractType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Resolve", (contractType))?;
        Ok(__cordl_ret)
    }
    pub fn HasBindingId_Object0<TContract>(
        &mut self,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasBindingId", (identifier))?;
        Ok(__cordl_ret)
    }
    pub fn HasBindingId_Type_Object1(
        &mut self,
        contractType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasBindingId", (contractType, identifier))?;
        Ok(__cordl_ret)
    }
    pub fn HasBindingId_Type_Object_InjectSources2(
        &mut self,
        contractType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
        sourceType: crate::Zenject::InjectSources,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasBindingId", (contractType, identifier, sourceType))?;
        Ok(__cordl_ret)
    }
    pub fn BindDisposableExecutionOrder_i32_0<T>(
        &mut self,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("BindDisposableExecutionOrder", (order))?;
        Ok(__cordl_ret)
    }
    pub fn BindDisposableExecutionOrder_Type_i32_1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("BindDisposableExecutionOrder", (_cordl_type, order))?;
        Ok(__cordl_ret)
    }
    pub fn BindInstances(
        &mut self,
        instances: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindInstances", (instances))?;
        Ok(__cordl_ret)
    }
    pub fn BindFactory_0<TContract, TFactory>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactory: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_1<TContract> = __cordl_object
            .invoke("BindFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactory_1<TParam1, TContract, TFactory>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_2<TParam1, TContract>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactory: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_2<
            TParam1,
            TContract,
        > = __cordl_object.invoke("BindFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactory_2<TParam1, TParam2, TContract, TFactory>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_3<TParam1, TParam2, TContract>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactory: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_3<
            TParam1,
            TParam2,
            TContract,
        > = __cordl_object.invoke("BindFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactory_3<TParam1, TParam2, TParam3, TContract, TFactory>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_4<
            TParam1,
            TParam2,
            TParam3,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactory: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_4<
            TParam1,
            TParam2,
            TParam3,
            TContract,
        > = __cordl_object.invoke("BindFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactory_4<TParam1, TParam2, TParam3, TParam4, TContract, TFactory>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_5<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactory: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_5<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TContract,
        > = __cordl_object.invoke("BindFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactory_5<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TContract,
        TFactory,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_6<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactory: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_6<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TContract,
        > = __cordl_object.invoke("BindFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactory_6<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TContract,
        TFactory,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_7<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactory: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_7<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TContract,
        > = __cordl_object.invoke("BindFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactory_7<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TParam8,
        TParam9,
        TParam10,
        TContract,
        TFactory,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_11<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TParam7,
            TParam8,
            TParam9,
            TParam10,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam7: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam8: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam9: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam10: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactory: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_11<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TParam7,
            TParam8,
            TParam9,
            TParam10,
            TContract,
        > = __cordl_object.invoke("BindFactory", ())?;
        Ok(__cordl_ret)
    }
    pub fn FlushBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FlushBindings", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryGetUniqueProvider(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::DiContainer_ProviderInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DiContainer_ProviderInfo = __cordl_object
            .invoke("TryGetUniqueProvider", (context))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponent_Object0<T>(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiatePrefabForComponent", (prefab))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponent_Object_IEnumerable_1_1<T>(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiatePrefabForComponent", (prefab, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponent_Object_Transform2<T>(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        parentTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiatePrefabForComponent", (prefab, parentTransform))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponent_Object_Transform_IEnumerable_1_3<T>(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        parentTransform: *mut crate::UnityEngine::Transform,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke(
                "InstantiatePrefabForComponent",
                (prefab, parentTransform, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponent_Object_Vector3_Quaternion_Transform4<T>(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke(
                "InstantiatePrefabForComponent",
                (prefab, position, rotation, parentTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponent_Object_Vector3_Quaternion_Transform_IEnumerable_1_5<
        T,
    >(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: *mut crate::UnityEngine::Transform,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke(
                "InstantiatePrefabForComponent",
                (prefab, position, rotation, parentTransform, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponent_Type_Object_Transform_IEnumerable_1_6(
        &mut self,
        concreteType: *mut crate::System::Type,
        prefab: *mut crate::UnityEngine::Object,
        parentTransform: *mut crate::UnityEngine::Transform,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InstantiatePrefabForComponent",
                (concreteType, prefab, parentTransform, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponent_Type_Object_IEnumerable_1_GameObjectCreationParameters7(
        &mut self,
        concreteType: *mut crate::System::Type,
        prefab: *mut crate::UnityEngine::Object,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
        creationInfo: *mut crate::Zenject::GameObjectCreationParameters,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InstantiatePrefabForComponent",
                (concreteType, prefab, extraArgs, creationInfo),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_Settings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ZenjectSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ZenjectSettings = __cordl_object
            .invoke("get_Settings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DefaultParent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_DefaultParent", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateFullResolve(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateFullResolve", ())?;
        Ok(__cordl_ret)
    }
    pub fn InjectMembersTopDown(
        &mut self,
        injectable: *mut crate::System::Object,
        injectableType: *mut crate::System::Type,
        typeInfo: *mut crate::Zenject::InjectTypeInfo,
        extraArgs: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        context: *mut crate::Zenject::InjectContext,
        concreteIdentifier: *mut crate::System::Object,
        isDryRun: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InjectMembersTopDown",
                (
                    injectable,
                    injectableType,
                    typeInfo,
                    extraArgs,
                    context,
                    concreteIdentifier,
                    isDryRun,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn BindFactoryInternal_0<TContract, TFactoryContract, TFactoryConcrete>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_1<TContract> = __cordl_object
            .invoke("BindFactoryInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactoryInternal_1<TParam1, TContract, TFactoryContract, TFactoryConcrete>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_2<TParam1, TContract>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_2<
            TParam1,
            TContract,
        > = __cordl_object.invoke("BindFactoryInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactoryInternal_2<
        TParam1,
        TParam2,
        TContract,
        TFactoryContract,
        TFactoryConcrete,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_3<TParam1, TParam2, TContract>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_3<
            TParam1,
            TParam2,
            TContract,
        > = __cordl_object.invoke("BindFactoryInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactoryInternal_3<
        TParam1,
        TParam2,
        TParam3,
        TContract,
        TFactoryContract,
        TFactoryConcrete,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_4<
            TParam1,
            TParam2,
            TParam3,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_4<
            TParam1,
            TParam2,
            TParam3,
            TContract,
        > = __cordl_object.invoke("BindFactoryInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactoryInternal_4<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TContract,
        TFactoryContract,
        TFactoryConcrete,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_5<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_5<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TContract,
        > = __cordl_object.invoke("BindFactoryInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactoryInternal_5<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TContract,
        TFactoryContract,
        TFactoryConcrete,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_6<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_6<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TContract,
        > = __cordl_object.invoke("BindFactoryInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactoryInternal_6<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TContract,
        TFactoryContract,
        TFactoryConcrete,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_7<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_7<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TContract,
        > = __cordl_object.invoke("BindFactoryInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactoryInternal_7<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TParam8,
        TParam9,
        TParam10,
        TContract,
        TFactoryContract,
        TFactoryConcrete,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_11<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TParam7,
            TParam8,
            TParam9,
            TParam10,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam7: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam8: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam9: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam10: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_11<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TParam7,
            TParam8,
            TParam9,
            TParam10,
            TContract,
        > = __cordl_object.invoke("BindFactoryInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn InjectExplicitInternal(
        &mut self,
        injectable: *mut crate::System::Object,
        injectableType: *mut crate::System::Type,
        extraArgs: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        context: *mut crate::Zenject::InjectContext,
        concreteIdentifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InjectExplicitInternal",
                (injectable, injectableType, extraArgs, context, concreteIdentifier),
            )?;
        Ok(__cordl_ret)
    }
    pub fn BindPoolableExecutionOrder_i32_0<T>(
        &mut self,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("BindPoolableExecutionOrder", (order))?;
        Ok(__cordl_ret)
    }
    pub fn BindPoolableExecutionOrder_Type_i32_1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("BindPoolableExecutionOrder", (_cordl_type, order))?;
        Ok(__cordl_ret)
    }
    pub fn Rebind_0<TContract>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ConcreteBinderGeneric_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConcreteBinderGeneric_1<TContract> = __cordl_object
            .invoke("Rebind", ())?;
        Ok(__cordl_ret)
    }
    pub fn Rebind_Type1(
        &mut self,
        contractType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConcreteBinderNonGeneric> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConcreteBinderNonGeneric = __cordl_object
            .invoke("Rebind", (contractType))?;
        Ok(__cordl_ret)
    }
    pub fn CheckForInstallWarning(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckForInstallWarning", (context))?;
        Ok(__cordl_ret)
    }
    pub fn FlushValidationQueue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FlushValidationQueue", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryResolveId_Object0<TContract>(
        &mut self,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<TContract>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TContract = __cordl_object
            .invoke("TryResolveId", (identifier))?;
        Ok(__cordl_ret)
    }
    pub fn TryResolveId_Type_Object1(
        &mut self,
        contractType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("TryResolveId", (contractType, identifier))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateComponent_GameObject0<TContract>(
        &mut self,
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<TContract>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TContract = __cordl_object
            .invoke("InstantiateComponent", (gameObject))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateComponent_GameObject_IEnumerable_1_1<TContract>(
        &mut self,
        gameObject: *mut crate::UnityEngine::GameObject,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<TContract>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TContract = __cordl_object
            .invoke("InstantiateComponent", (gameObject, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateComponent_Type_GameObject2(
        &mut self,
        componentType: *mut crate::System::Type,
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("InstantiateComponent", (componentType, gameObject))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateComponent_Type_GameObject_IEnumerable_1_3(
        &mut self,
        componentType: *mut crate::System::Type,
        gameObject: *mut crate::UnityEngine::GameObject,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("InstantiateComponent", (componentType, gameObject, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn FinalizeBinding(
        &mut self,
        binding: *mut crate::Zenject::BindStatement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinalizeBinding", (binding))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetDecoratorProvider(
        &mut self,
        contractType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::Internal::IDecoratorProvider,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::Internal::IDecoratorProvider = __cordl_object
            .invoke("TryGetDecoratorProvider", (contractType))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefab_Object0(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("InstantiatePrefab", (prefab))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefab_Transform1(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        parentTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("InstantiatePrefab", (prefab, parentTransform))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefab_Vector3_Quaternion_Transform2(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("InstantiatePrefab", (prefab, position, rotation, parentTransform))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefab_GameObjectCreationParameters3(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("InstantiatePrefab", (prefab, gameObjectBindInfo))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateComponentExplicit(
        &mut self,
        componentType: *mut crate::System::Type,
        gameObject: *mut crate::UnityEngine::GameObject,
        extraArgs: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke(
                "InstantiateComponentExplicit",
                (componentType, gameObject, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_SingletonMarkRegistry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::Internal::SingletonMarkRegistry,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::Internal::SingletonMarkRegistry = __cordl_object
            .invoke("get_SingletonMarkRegistry", ())?;
        Ok(__cordl_ret)
    }
    pub fn RebindId_Object0<TContract>(
        &mut self,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ConcreteBinderGeneric_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConcreteBinderGeneric_1<TContract> = __cordl_object
            .invoke("RebindId", (identifier))?;
        Ok(__cordl_ret)
    }
    pub fn RebindId_Type_Object1(
        &mut self,
        contractType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConcreteBinderNonGeneric> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConcreteBinderNonGeneric = __cordl_object
            .invoke("RebindId", (contractType, identifier))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateScriptableObjectResource_String0<T>(
        &mut self,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiateScriptableObjectResource", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateScriptableObjectResource_String_IEnumerable_1_1<T>(
        &mut self,
        resourcePath: *mut crate::System::String,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiateScriptableObjectResource", (resourcePath, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateScriptableObjectResource_Type_String2(
        &mut self,
        scriptableObjectType: *mut crate::System::Type,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InstantiateScriptableObjectResource",
                (scriptableObjectType, resourcePath),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateScriptableObjectResource_Type_String_IEnumerable_1_3(
        &mut self,
        scriptableObjectType: *mut crate::System::Type,
        resourcePath: *mut crate::System::String,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InstantiateScriptableObjectResource",
                (scriptableObjectType, resourcePath, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateAndParentPrefab(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
        context: *mut crate::Zenject::InjectContext,
        shouldMakeActive: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke(
                "CreateAndParentPrefab",
                (prefab, gameObjectBindInfo, context, shouldMakeActive),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_AssertOnNewGameObjects(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AssertOnNewGameObjects", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResolveTypeAll_Type0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::Type>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Type,
        > = __cordl_object.invoke("ResolveTypeAll", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveTypeAll_Type_Object1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::Type>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Type,
        > = __cordl_object.invoke("ResolveTypeAll", (_cordl_type, identifier))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveTypeAll_InjectContext2(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::Type>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Type,
        > = __cordl_object.invoke("ResolveTypeAll", (context))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveType_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("ResolveType", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResolveType_Type1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("ResolveType", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveType_InjectContext2(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("ResolveType", (context))?;
        Ok(__cordl_ret)
    }
    pub fn GetProviderMatches(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
        buffer: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::DiContainer_ProviderInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetProviderMatches", (context, buffer))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveId_Object0<TContract>(
        &mut self,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<TContract>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TContract = __cordl_object.invoke("ResolveId", (identifier))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveId_Type_Object1(
        &mut self,
        contractType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ResolveId", (contractType, identifier))?;
        Ok(__cordl_ret)
    }
    pub fn Bind_0<TContract>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ConcreteIdBinderGeneric_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConcreteIdBinderGeneric_1<TContract> = __cordl_object
            .invoke("Bind", ())?;
        Ok(__cordl_ret)
    }
    pub fn Bind_BindStatement1<TContract>(
        &mut self,
        bindStatement: *mut crate::Zenject::BindStatement,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ConcreteIdBinderGeneric_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConcreteIdBinderGeneric_1<TContract> = __cordl_object
            .invoke("Bind", (bindStatement))?;
        Ok(__cordl_ret)
    }
    pub fn Bind_Il2CppArray2(
        &mut self,
        contractTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConcreteIdBinderNonGeneric> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConcreteIdBinderNonGeneric = __cordl_object
            .invoke("Bind", (contractTypes))?;
        Ok(__cordl_ret)
    }
    pub fn Bind_IEnumerable_1_3(
        &mut self,
        contractTypes: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Type,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConcreteIdBinderNonGeneric> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConcreteIdBinderNonGeneric = __cordl_object
            .invoke("Bind", (contractTypes))?;
        Ok(__cordl_ret)
    }
    pub fn Bind_Action_1_4(
        &mut self,
        generator: *mut crate::System::Action_1<
            *mut crate::Zenject::ConventionSelectTypesBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConcreteIdBinderNonGeneric> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConcreteIdBinderNonGeneric = __cordl_object
            .invoke("Bind", (generator))?;
        Ok(__cordl_ret)
    }
    pub fn QueueForValidate(
        &mut self,
        validatable: *mut crate::Zenject::IValidatable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueForValidate", (validatable))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveDependencyRoots(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResolveDependencyRoots", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindInitializableExecutionOrder_i32_0<T>(
        &mut self,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("BindInitializableExecutionOrder", (order))?;
        Ok(__cordl_ret)
    }
    pub fn BindInitializableExecutionOrder_Type_i32_1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("BindInitializableExecutionOrder", (_cordl_type, order))?;
        Ok(__cordl_ret)
    }
    pub fn set_Settings(
        &mut self,
        value: *mut crate::Zenject::ZenjectSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Settings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveAll_InjectContext0(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("ResolveAll", (context))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveAll_InjectContext_List_1_1(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
        buffer: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResolveAll", (context, buffer))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveAll_2<TContract>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<TContract> = __cordl_object
            .invoke("ResolveAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResolveAll_Type3(
        &mut self,
        contractType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("ResolveAll", (contractType))?;
        Ok(__cordl_ret)
    }
    pub fn set_IsInstalling(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsInstalling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Decorate<TContract>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::DecoratorToChoiceFromBinder_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DecoratorToChoiceFromBinder_1<TContract> = __cordl_object
            .invoke("Decorate", ())?;
        Ok(__cordl_ret)
    }
    pub fn CallInjectMethodsTopDown(
        &mut self,
        injectable: *mut crate::System::Object,
        injectableType: *mut crate::System::Type,
        typeInfo: *mut crate::Zenject::InjectTypeInfo,
        extraArgs: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        context: *mut crate::Zenject::InjectContext,
        concreteIdentifier: *mut crate::System::Object,
        isDryRun: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CallInjectMethodsTopDown",
                (
                    injectable,
                    injectableType,
                    typeInfo,
                    extraArgs,
                    context,
                    concreteIdentifier,
                    isDryRun,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InjectGameObject(
        &mut self,
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InjectGameObject", (gameObject))?;
        Ok(__cordl_ret)
    }
    pub fn UnbindId_Object0<TContract>(
        &mut self,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("UnbindId", (identifier))?;
        Ok(__cordl_ret)
    }
    pub fn UnbindId_Type_Object1(
        &mut self,
        contractType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnbindId", (contractType, identifier))?;
        Ok(__cordl_ret)
    }
    pub fn UnbindId_Object2<TContract, TConcrete>(
        &mut self,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("UnbindId", (identifier))?;
        Ok(__cordl_ret)
    }
    pub fn UnbindId_Type_Type_Object3(
        &mut self,
        contractType: *mut crate::System::Type,
        concreteType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnbindId", (contractType, concreteType, identifier))?;
        Ok(__cordl_ret)
    }
    pub fn BindMemoryPoolCustomInterfaceNoFlush<
        TItemContract,
        TPoolConcrete,
        TPoolContract,
    >(
        &mut self,
        includeConcreteType: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::MemoryPoolIdInitialSizeMaxSizeBinder_1<TItemContract>,
    >
    where
        TItemContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TPoolConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TPoolContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::MemoryPoolIdInitialSizeMaxSizeBinder_1<
            TItemContract,
        > = __cordl_object
            .invoke("BindMemoryPoolCustomInterfaceNoFlush", (includeConcreteType))?;
        Ok(__cordl_ret)
    }
    pub fn BindMemoryPoolCustomInterface<TItemContract, TPoolConcrete, TPoolContract>(
        &mut self,
        includeConcreteType: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::MemoryPoolIdInitialSizeMaxSizeBinder_1<TItemContract>,
    >
    where
        TItemContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TPoolConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TPoolContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::MemoryPoolIdInitialSizeMaxSizeBinder_1<
            TItemContract,
        > = __cordl_object
            .invoke("BindMemoryPoolCustomInterface", (includeConcreteType))?;
        Ok(__cordl_ret)
    }
    pub fn get_AllProviders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::IProvider,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::IProvider,
        > = __cordl_object.invoke("get_AllProviders", ())?;
        Ok(__cordl_ret)
    }
    pub fn Inject_Object0(
        &mut self,
        injectable: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Inject", (injectable))?;
        Ok(__cordl_ret)
    }
    pub fn Inject_IEnumerable_1_1(
        &mut self,
        injectable: *mut crate::System::Object,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Inject", (injectable, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn BindTickableExecutionOrder_i32_0<T>(
        &mut self,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("BindTickableExecutionOrder", (order))?;
        Ok(__cordl_ret)
    }
    pub fn BindTickableExecutionOrder_Type_i32_1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("BindTickableExecutionOrder", (_cordl_type, order))?;
        Ok(__cordl_ret)
    }
    pub fn GetDecoratedInstances(
        &mut self,
        provider: *mut crate::Zenject::IProvider,
        context: *mut crate::Zenject::InjectContext,
        buffer: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetDecoratedInstances", (provider, context, buffer))?;
        Ok(__cordl_ret)
    }
    pub fn get_AllContracts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::Zenject::BindingId,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::Zenject::BindingId,
        > = __cordl_object.invoke("get_AllContracts", ())?;
        Ok(__cordl_ret)
    }
    pub fn Install_0<TInstaller>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TInstaller: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Install", ())?;
        Ok(__cordl_ret)
    }
    pub fn Install_Il2CppArray1<TInstaller>(
        &mut self,
        extraArgs: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TInstaller: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Install", (extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn CreateAndParentPrefabResource(
        &mut self,
        resourcePath: *mut crate::System::String,
        gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
        context: *mut crate::Zenject::InjectContext,
        shouldMakeActive: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke(
                "CreateAndParentPrefabResource",
                (resourcePath, gameObjectBindInfo, context, shouldMakeActive),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RegisterProvider(
        &mut self,
        bindingId: crate::Zenject::BindingId,
        condition: *mut crate::Zenject::BindingCondition,
        provider: *mut crate::Zenject::IProvider,
        nonLazy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterProvider", (bindingId, condition, provider, nonLazy))?;
        Ok(__cordl_ret)
    }
    pub fn get_AncestorContainers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::Zenject::DiContainer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Zenject::DiContainer,
        > = __cordl_object.invoke("get_AncestorContainers", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindInstance<TContract>(
        &mut self,
        instance: TContract,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::IdScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::IdScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("BindInstance", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn BindExecutionOrder_i32_0<T>(
        &mut self,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindExecutionOrder", (order))?;
        Ok(__cordl_ret)
    }
    pub fn BindExecutionOrder_Type_i32_1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindExecutionOrder", (_cordl_type, order))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResourceForComponent_String0<T>(
        &mut self,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiatePrefabResourceForComponent", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResourceForComponent_String_IEnumerable_1_1<T>(
        &mut self,
        resourcePath: *mut crate::System::String,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiatePrefabResourceForComponent", (resourcePath, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResourceForComponent_String_Transform2<T>(
        &mut self,
        resourcePath: *mut crate::System::String,
        parentTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke(
                "InstantiatePrefabResourceForComponent",
                (resourcePath, parentTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResourceForComponent_String_Transform_IEnumerable_1_3<T>(
        &mut self,
        resourcePath: *mut crate::System::String,
        parentTransform: *mut crate::UnityEngine::Transform,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke(
                "InstantiatePrefabResourceForComponent",
                (resourcePath, parentTransform, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResourceForComponent_String_Vector3_Quaternion_Transform4<T>(
        &mut self,
        resourcePath: *mut crate::System::String,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke(
                "InstantiatePrefabResourceForComponent",
                (resourcePath, position, rotation, parentTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResourceForComponent_String_Vector3_Quaternion_Transform_IEnumerable_1_5<
        T,
    >(
        &mut self,
        resourcePath: *mut crate::System::String,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: *mut crate::UnityEngine::Transform,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke(
                "InstantiatePrefabResourceForComponent",
                (resourcePath, position, rotation, parentTransform, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResourceForComponent_Type_String_Transform_IEnumerable_1_6(
        &mut self,
        concreteType: *mut crate::System::Type,
        resourcePath: *mut crate::System::String,
        parentTransform: *mut crate::UnityEngine::Transform,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InstantiatePrefabResourceForComponent",
                (concreteType, resourcePath, parentTransform, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn BindLateDisposableExecutionOrder_i32_0<T>(
        &mut self,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("BindLateDisposableExecutionOrder", (order))?;
        Ok(__cordl_ret)
    }
    pub fn BindLateDisposableExecutionOrder_Type_i32_1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("BindLateDisposableExecutionOrder", (_cordl_type, order))?;
        Ok(__cordl_ret)
    }
    pub fn get_InheritedDefaultParent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_InheritedDefaultParent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsValidating(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsValidating", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateLazyBinding(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("CreateLazyBinding", (context))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsInstalling(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInstalling", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTransformGroup(
        &mut self,
        gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
        context: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("GetTransformGroup", (gameObjectBindInfo, context))?;
        Ok(__cordl_ret)
    }
    pub fn CreateSubContainer_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::DiContainer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DiContainer = __cordl_object
            .invoke("CreateSubContainer", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateSubContainer__cordl_bool1(
        &mut self,
        isValidating: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::DiContainer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DiContainer = __cordl_object
            .invoke("CreateSubContainer", (isValidating))?;
        Ok(__cordl_ret)
    }
    pub fn CreateEmptyGameObject_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("CreateEmptyGameObject", (name))?;
        Ok(__cordl_ret)
    }
    pub fn CreateEmptyGameObject_GameObjectCreationParameters_InjectContext1(
        &mut self,
        gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
        context: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("CreateEmptyGameObject", (gameObjectBindInfo, context))?;
        Ok(__cordl_ret)
    }
    pub fn UnbindInterfacesTo_0<TConcrete>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnbindInterfacesTo", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnbindInterfacesTo_Type1(
        &mut self,
        concreteType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnbindInterfacesTo", (concreteType))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateComponentOnNewGameObject_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiateComponentOnNewGameObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateComponentOnNewGameObject_IEnumerable_1_1<T>(
        &mut self,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiateComponentOnNewGameObject", (extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateComponentOnNewGameObject_String2<T>(
        &mut self,
        gameObjectName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiateComponentOnNewGameObject", (gameObjectName))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateComponentOnNewGameObject_String_IEnumerable_1_3<T>(
        &mut self,
        gameObjectName: *mut crate::System::String,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiateComponentOnNewGameObject", (gameObjectName, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn BindFactoryCustomInterface_0<TContract, TFactoryConcrete, TFactoryContract>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_1<TContract>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_1<TContract> = __cordl_object
            .invoke("BindFactoryCustomInterface", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactoryCustomInterface_1<
        TParam1,
        TContract,
        TFactoryConcrete,
        TFactoryContract,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_2<TParam1, TContract>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_2<
            TParam1,
            TContract,
        > = __cordl_object.invoke("BindFactoryCustomInterface", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactoryCustomInterface_2<
        TParam1,
        TParam2,
        TContract,
        TFactoryConcrete,
        TFactoryContract,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_3<TParam1, TParam2, TContract>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_3<
            TParam1,
            TParam2,
            TContract,
        > = __cordl_object.invoke("BindFactoryCustomInterface", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactoryCustomInterface_3<
        TParam1,
        TParam2,
        TParam3,
        TContract,
        TFactoryConcrete,
        TFactoryContract,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_4<
            TParam1,
            TParam2,
            TParam3,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_4<
            TParam1,
            TParam2,
            TParam3,
            TContract,
        > = __cordl_object.invoke("BindFactoryCustomInterface", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactoryCustomInterface_4<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TContract,
        TFactoryConcrete,
        TFactoryContract,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_5<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_5<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TContract,
        > = __cordl_object.invoke("BindFactoryCustomInterface", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactoryCustomInterface_5<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TContract,
        TFactoryConcrete,
        TFactoryContract,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_6<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_6<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TContract,
        > = __cordl_object.invoke("BindFactoryCustomInterface", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactoryCustomInterface_6<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TContract,
        TFactoryConcrete,
        TFactoryContract,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_7<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_7<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TContract,
        > = __cordl_object.invoke("BindFactoryCustomInterface", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindFactoryCustomInterface_7<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TParam8,
        TParam9,
        TParam10,
        TContract,
        TFactoryConcrete,
        TFactoryContract,
    >(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::FactoryToChoiceIdBinder_11<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TParam7,
            TParam8,
            TParam9,
            TParam10,
            TContract,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam7: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam8: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam9: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam10: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactoryContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryToChoiceIdBinder_11<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TParam7,
            TParam8,
            TParam9,
            TParam10,
            TContract,
        > = __cordl_object.invoke("BindFactoryCustomInterface", ())?;
        Ok(__cordl_ret)
    }
    pub fn Unbind_0<TContract>(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Unbind", ())?;
        Ok(__cordl_ret)
    }
    pub fn Unbind_Type1(
        &mut self,
        contractType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Unbind", (contractType))?;
        Ok(__cordl_ret)
    }
    pub fn Unbind_2<TContract, TConcrete>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Unbind", ())?;
        Ok(__cordl_ret)
    }
    pub fn Unbind_Type_Type3(
        &mut self,
        contractType: *mut crate::System::Type,
        concreteType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Unbind", (contractType, concreteType))?;
        Ok(__cordl_ret)
    }
    pub fn GetContainerHeirarchyDistance_DiContainer0(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetContainerHeirarchyDistance", (container))?;
        Ok(__cordl_ret)
    }
    pub fn GetContainerHeirarchyDistance_i32_1(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<i32> = __cordl_object
            .invoke("GetContainerHeirarchyDistance", (container, depth))?;
        Ok(__cordl_ret)
    }
    pub fn get_ContextTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_ContextTransform", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindInterfacesTo_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::FromBinderNonGeneric>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FromBinderNonGeneric = __cordl_object
            .invoke("BindInterfacesTo", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindInterfacesTo_Type1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::FromBinderNonGeneric> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FromBinderNonGeneric = __cordl_object
            .invoke("BindInterfacesTo", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn BindLateTickableExecutionOrder_i32_0<T>(
        &mut self,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("BindLateTickableExecutionOrder", (order))?;
        Ok(__cordl_ret)
    }
    pub fn BindLateTickableExecutionOrder_Type_i32_1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("BindLateTickableExecutionOrder", (_cordl_type, order))?;
        Ok(__cordl_ret)
    }
    pub fn InstallDefaultBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallDefaultBindings", ())?;
        Ok(__cordl_ret)
    }
    pub fn ShouldInheritBinding(
        &mut self,
        binding: *mut crate::Zenject::BindStatement,
        ancestorContainer: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldInheritBinding", (binding, ancestorContainer))?;
        Ok(__cordl_ret)
    }
    pub fn QueueForInject(
        &mut self,
        instance: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueForInject", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn New_IEnumerable_1__cordl_bool0(
        parentContainersEnumerable: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::DiContainer,
        >,
        isValidating: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parentContainersEnumerable, isValidating))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool1(
        isValidating: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (isValidating))?;
        Ok(__cordl_object)
    }
    pub fn New_2() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_DiContainer__cordl_bool3(
        parentContainer: *mut crate::Zenject::DiContainer,
        isValidating: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parentContainer, isValidating))?;
        Ok(__cordl_object)
    }
    pub fn New_DiContainer4(
        parentContainer: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parentContainer))?;
        Ok(__cordl_object)
    }
    pub fn New_IEnumerable_1_5(
        parentContainers: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::DiContainer,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parentContainers))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+DiContainer")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::DiContainer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+DiContainer+ProviderInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct DiContainer_ProviderInfo {
    __cordl_parent: crate::System::Object,
    pub Container: *mut crate::Zenject::DiContainer,
    pub NonLazy: bool,
    pub Provider: *mut crate::Zenject::IProvider,
    pub Condition: *mut crate::Zenject::BindingCondition,
}
#[cfg(feature = "Zenject+DiContainer+ProviderInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::DiContainer_ProviderInfo => "Zenject"
    ."DiContainer/ProviderInfo"
);
#[cfg(feature = "Zenject+DiContainer+ProviderInfo")]
impl std::ops::Deref for crate::Zenject::DiContainer_ProviderInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DiContainer+ProviderInfo")]
impl std::ops::DerefMut for crate::Zenject::DiContainer_ProviderInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DiContainer+ProviderInfo")]
impl crate::Zenject::DiContainer_ProviderInfo {
    pub fn _ctor(
        &mut self,
        provider: *mut crate::Zenject::IProvider,
        condition: *mut crate::Zenject::BindingCondition,
        nonLazy: bool,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (provider, condition, nonLazy, container))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        provider: *mut crate::Zenject::IProvider,
        condition: *mut crate::Zenject::BindingCondition,
        nonLazy: bool,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (provider, condition, nonLazy, container))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+DiContainer+ProviderInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::DiContainer_ProviderInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
