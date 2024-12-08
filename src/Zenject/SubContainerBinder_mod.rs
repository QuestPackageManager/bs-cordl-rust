#[cfg(feature = "Zenject+SubContainerBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerBinder {
    __cordl_parent: crate::System::Object,
    pub _bindInfo: *mut crate::Zenject::BindInfo,
    pub _bindStatement: *mut crate::Zenject::BindStatement,
    pub _subIdentifier: *mut crate::System::Object,
    pub _resolveAll: bool,
}
#[cfg(feature = "Zenject+SubContainerBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SubContainerBinder => "Zenject"
    ."SubContainerBinder"
);
#[cfg(feature = "Zenject+SubContainerBinder")]
impl std::ops::Deref for crate::Zenject::SubContainerBinder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerBinder")]
impl std::ops::DerefMut for crate::Zenject::SubContainerBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerBinder")]
impl crate::Zenject::SubContainerBinder {
    #[cfg(feature = "Zenject+SubContainerBinder+__c__DisplayClass10_0")]
    pub type __c__DisplayClass10_0 = crate::Zenject::SubContainerBinder___c__DisplayClass10_0;
    #[cfg(feature = "Zenject+SubContainerBinder+__c__DisplayClass11_0")]
    pub type __c__DisplayClass11_0 = crate::Zenject::SubContainerBinder___c__DisplayClass11_0;
    #[cfg(feature = "Zenject+SubContainerBinder+__c__DisplayClass12_0")]
    pub type __c__DisplayClass12_0 = crate::Zenject::SubContainerBinder___c__DisplayClass12_0;
    #[cfg(feature = "Zenject+SubContainerBinder+__c__DisplayClass13_0")]
    pub type __c__DisplayClass13_0 = crate::Zenject::SubContainerBinder___c__DisplayClass13_0;
    #[cfg(feature = "Zenject+SubContainerBinder+__c__DisplayClass15_0")]
    pub type __c__DisplayClass15_0 = crate::Zenject::SubContainerBinder___c__DisplayClass15_0;
    #[cfg(feature = "Zenject+SubContainerBinder+__c__DisplayClass17_0")]
    pub type __c__DisplayClass17_0 = crate::Zenject::SubContainerBinder___c__DisplayClass17_0;
    #[cfg(feature = "Zenject+SubContainerBinder+__c__DisplayClass18_0")]
    pub type __c__DisplayClass18_0 = crate::Zenject::SubContainerBinder___c__DisplayClass18_0;
    #[cfg(feature = "Zenject+SubContainerBinder+__c__DisplayClass20_0")]
    pub type __c__DisplayClass20_0 = crate::Zenject::SubContainerBinder___c__DisplayClass20_0;
    #[cfg(feature = "Zenject+SubContainerBinder+__c__DisplayClass22_0")]
    pub type __c__DisplayClass22_0 = crate::Zenject::SubContainerBinder___c__DisplayClass22_0;
    #[cfg(feature = "Zenject+SubContainerBinder+__c__DisplayClass24_0")]
    pub type __c__DisplayClass24_0 = crate::Zenject::SubContainerBinder___c__DisplayClass24_0;
    #[cfg(feature = "Zenject+SubContainerBinder+__c__DisplayClass7_0")]
    pub type __c__DisplayClass7_0 = crate::Zenject::SubContainerBinder___c__DisplayClass7_0;
    #[cfg(feature = "Zenject+SubContainerBinder+__c__DisplayClass8_0")]
    pub type __c__DisplayClass8_0 = crate::Zenject::SubContainerBinder___c__DisplayClass8_0;
    pub fn ByInstaller_0<TInstaller>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TInstaller: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByInstaller", ())?;
        Ok(__cordl_ret)
    }
    pub fn ByInstaller_Type1(
        &mut self,
        installerType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByInstaller", (installerType))?;
        Ok(__cordl_ret)
    }
    pub fn ByInstance(
        &mut self,
        subContainer: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByInstance", (subContainer))?;
        Ok(__cordl_ret)
    }
    pub fn ByInstanceGetter(
        &mut self,
        subContainerGetter: *mut crate::System::Func_2<
            *mut crate::Zenject::InjectContext,
            *mut crate::Zenject::DiContainer,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByInstanceGetter", (subContainerGetter))?;
        Ok(__cordl_ret)
    }
    pub fn ByMethod(
        &mut self,
        installerMethod: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::WithKernelDefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByMethod", (installerMethod))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewContextPrefab(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewContextPrefab", (prefab))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewContextPrefabResource(
        &mut self,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewContextPrefabResource", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewGameObjectInstaller_0<TInstaller>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TInstaller: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewGameObjectInstaller", ())?;
        Ok(__cordl_ret)
    }
    pub fn ByNewGameObjectInstaller_Type1(
        &mut self,
        installerType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewGameObjectInstaller", (installerType))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewGameObjectMethod(
        &mut self,
        installerMethod: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewGameObjectMethod", (installerMethod))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewPrefab(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewPrefab", (prefab))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewPrefabInstaller_Object0<TInstaller>(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TInstaller: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewPrefabInstaller", (prefab))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewPrefabInstaller_Type1(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        installerType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewPrefabInstaller", (prefab, installerType))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewPrefabMethod(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        installerMethod: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewPrefabMethod", (prefab, installerMethod))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewPrefabResource(
        &mut self,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewPrefabResource", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewPrefabResourceInstaller_String0<TInstaller>(
        &mut self,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    >
    where
        TInstaller: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewPrefabResourceInstaller", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewPrefabResourceInstaller_Type1(
        &mut self,
        resourcePath: *mut crate::System::String,
        installerType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewPrefabResourceInstaller", (resourcePath, installerType))?;
        Ok(__cordl_ret)
    }
    pub fn ByNewPrefabResourceMethod(
        &mut self,
        resourcePath: *mut crate::System::String,
        installerMethod: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("ByNewPrefabResourceMethod", (resourcePath, installerMethod))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bindInfo: *mut crate::Zenject::BindInfo,
        bindStatement: *mut crate::Zenject::BindStatement,
        subIdentifier: *mut crate::System::Object,
        resolveAll: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo, bindStatement, subIdentifier, resolveAll))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        bindInfo: *mut crate::Zenject::BindInfo,
        bindStatement: *mut crate::Zenject::BindStatement,
        subIdentifier: *mut crate::System::Object,
        resolveAll: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindInfo, bindStatement, subIdentifier, resolveAll))?;
        Ok(__cordl_ret)
    }
    pub fn set_SubFinalizer(
        &mut self,
        value: *mut crate::Zenject::IBindingFinalizer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SubFinalizer", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+SubContainerBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::SubContainerBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
