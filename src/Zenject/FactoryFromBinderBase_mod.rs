#[cfg(feature = "Zenject+FactoryFromBinderBase")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryFromBinderBase {
    __cordl_parent: crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    pub _BindContainer_k__BackingField: *mut crate::Zenject::DiContainer,
    pub _FactoryBindInfo_k__BackingField: *mut crate::Zenject::FactoryBindInfo,
    pub _ContractType_k__BackingField: *mut crate::System::Type,
}
#[cfg(feature = "Zenject+FactoryFromBinderBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactoryFromBinderBase => "Zenject"
    ."FactoryFromBinderBase"
);
#[cfg(feature = "Zenject+FactoryFromBinderBase")]
impl std::ops::Deref for crate::Zenject::FactoryFromBinderBase {
    type Target = crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinderBase")]
impl std::ops::DerefMut for crate::Zenject::FactoryFromBinderBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinderBase")]
impl crate::Zenject::FactoryFromBinderBase {
    #[cfg(feature = "Zenject+FactoryFromBinderBase+__c__DisplayClass33_0")]
    pub type __c__DisplayClass33_0 = crate::Zenject::FactoryFromBinderBase___c__DisplayClass33_0;
    #[cfg(feature = "Zenject+FactoryFromBinderBase+__c__DisplayClass34_0")]
    pub type __c__DisplayClass34_0 = crate::Zenject::FactoryFromBinderBase___c__DisplayClass34_0;
    #[cfg(feature = "Zenject+FactoryFromBinderBase+__c__DisplayClass26_0")]
    pub type __c__DisplayClass26_0 = crate::Zenject::FactoryFromBinderBase___c__DisplayClass26_0;
    #[cfg(feature = "Zenject+FactoryFromBinderBase+_get_AllParentTypes_d__17")]
    pub type _get_AllParentTypes_d__17 = crate::Zenject::FactoryFromBinderBase__get_AllParentTypes_d__17;
    #[cfg(feature = "Zenject+FactoryFromBinderBase+__c__DisplayClass28_0")]
    pub type __c__DisplayClass28_0 = crate::Zenject::FactoryFromBinderBase___c__DisplayClass28_0;
    #[cfg(feature = "Zenject+FactoryFromBinderBase+__c__DisplayClass31_0")]
    pub type __c__DisplayClass31_0 = crate::Zenject::FactoryFromBinderBase___c__DisplayClass31_0;
    #[cfg(feature = "Zenject+FactoryFromBinderBase+__c__DisplayClass20_0")]
    pub type __c__DisplayClass20_0 = crate::Zenject::FactoryFromBinderBase___c__DisplayClass20_0;
    #[cfg(feature = "Zenject+FactoryFromBinderBase+__c__DisplayClass24_0")]
    pub type __c__DisplayClass24_0 = crate::Zenject::FactoryFromBinderBase___c__DisplayClass24_0;
    #[cfg(feature = "Zenject+FactoryFromBinderBase+__c__DisplayClass21_0")]
    pub type __c__DisplayClass21_0 = crate::Zenject::FactoryFromBinderBase___c__DisplayClass21_0;
    #[cfg(feature = "Zenject+FactoryFromBinderBase+__c__DisplayClass23_0")]
    pub type __c__DisplayClass23_0 = crate::Zenject::FactoryFromBinderBase___c__DisplayClass23_0;
    #[cfg(feature = "Zenject+FactoryFromBinderBase+__c__DisplayClass27_0")]
    pub type __c__DisplayClass27_0 = crate::Zenject::FactoryFromBinderBase___c__DisplayClass27_0;
    #[cfg(feature = "Zenject+FactoryFromBinderBase+__c__DisplayClass29_0")]
    pub type __c__DisplayClass29_0 = crate::Zenject::FactoryFromBinderBase___c__DisplayClass29_0;
    #[cfg(feature = "Zenject+FactoryFromBinderBase+__c__DisplayClass30_0")]
    pub type __c__DisplayClass30_0 = crate::Zenject::FactoryFromBinderBase___c__DisplayClass30_0;
    #[cfg(feature = "Zenject+FactoryFromBinderBase+__c__DisplayClass32_0")]
    pub type __c__DisplayClass32_0 = crate::Zenject::FactoryFromBinderBase___c__DisplayClass32_0;
    pub fn FromInstance(
        &mut self,
        instance: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromInstance", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn FromNew(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromNew", ())?;
        Ok(__cordl_ret)
    }
    pub fn FromNewScriptableObjectResource(
        &mut self,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromNewScriptableObjectResource", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn _FromComponentOnRoot_b__25_0(
        &mut self,
        ctx: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("<FromComponentOnRoot>b__25_0", (ctx))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bindContainer: *mut crate::Zenject::DiContainer,
        contractType: *mut crate::System::Type,
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindContainer, contractType, bindInfo, factoryBindInfo))?;
        Ok(__cordl_ret)
    }
    pub fn get_ContractType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ContractType", ())?;
        Ok(__cordl_ret)
    }
    pub fn FromNewComponentOnNewPrefab(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromNewComponentOnNewPrefab", (prefab))?;
        Ok(__cordl_ret)
    }
    pub fn set_FactoryBindInfo(
        &mut self,
        value: *mut crate::Zenject::FactoryBindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FactoryBindInfo", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ProviderFunc(
        &mut self,
        value: *mut crate::System::Func_2<
            *mut crate::Zenject::DiContainer,
            *mut crate::Zenject::IProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ProviderFunc", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_FactoryBindInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::FactoryBindInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FactoryBindInfo = __cordl_object
            .invoke("get_FactoryBindInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn FromComponentOnRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromComponentOnRoot", ())?;
        Ok(__cordl_ret)
    }
    pub fn FromComponentInNewPrefabResource(
        &mut self,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromComponentInNewPrefabResource", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn set_BindContainer(
        &mut self,
        value: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BindContainer", (value))?;
        Ok(__cordl_ret)
    }
    pub fn FromComponentOn_GameObject0(
        &mut self,
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromComponentOn", (gameObject))?;
        Ok(__cordl_ret)
    }
    pub fn FromComponentOn_Func_2_1(
        &mut self,
        gameObjectGetter: *mut crate::System::Func_2<
            *mut crate::Zenject::InjectContext,
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromComponentOn", (gameObjectGetter))?;
        Ok(__cordl_ret)
    }
    pub fn get_ProviderFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Func_2<
            *mut crate::Zenject::DiContainer,
            *mut crate::Zenject::IProvider,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Func_2<
            *mut crate::Zenject::DiContainer,
            *mut crate::Zenject::IProvider,
        > = __cordl_object.invoke("get_ProviderFunc", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_BindContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::DiContainer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DiContainer = __cordl_object
            .invoke("get_BindContainer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AllParentTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Type,
        > = __cordl_object.invoke("get_AllParentTypes", ())?;
        Ok(__cordl_ret)
    }
    pub fn FromComponentInNewPrefab(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromComponentInNewPrefab", (prefab))?;
        Ok(__cordl_ret)
    }
    pub fn FromNewComponentOnNewPrefabResource(
        &mut self,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromNewComponentOnNewPrefabResource", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn FromNewComponentOn_GameObject0(
        &mut self,
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromNewComponentOn", (gameObject))?;
        Ok(__cordl_ret)
    }
    pub fn FromNewComponentOn_Func_2_1(
        &mut self,
        gameObjectGetter: *mut crate::System::Func_2<
            *mut crate::Zenject::InjectContext,
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromNewComponentOn", (gameObjectGetter))?;
        Ok(__cordl_ret)
    }
    pub fn FromResolve_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromResolve", ())?;
        Ok(__cordl_ret)
    }
    pub fn FromResolve_Object1(
        &mut self,
        subIdentifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromResolve", (subIdentifier))?;
        Ok(__cordl_ret)
    }
    pub fn FromResource(
        &mut self,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromResource", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn __ctor_b__0_0(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::IProvider> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::IProvider = __cordl_object
            .invoke("<.ctor>b__0_0", (container))?;
        Ok(__cordl_ret)
    }
    pub fn CreateIFactoryBinder<T>(
        &mut self,
        factoryId: quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConcreteBinderGeneric_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConcreteBinderGeneric_1<T> = __cordl_object
            .invoke("CreateIFactoryBinder", (factoryId))?;
        Ok(__cordl_ret)
    }
    pub fn set_ContractType(
        &mut self,
        value: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ContractType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn FromScriptableObjectResource(
        &mut self,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("FromScriptableObjectResource", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bindContainer: *mut crate::Zenject::DiContainer,
        contractType: *mut crate::System::Type,
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (bindContainer, contractType, bindInfo, factoryBindInfo),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+FactoryFromBinderBase")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::FactoryFromBinderBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
