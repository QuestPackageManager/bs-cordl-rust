#[cfg(feature = "Zenject+DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder {
    __cordl_parent: crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    pub _SubContainerCreatorBindInfo_k__BackingField: *mut crate::Zenject::SubContainerCreatorBindInfo,
}
#[cfg(feature = "Zenject+DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder => "Zenject"
    ."DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder"
);
#[cfg(feature = "Zenject+DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder")]
impl std::ops::Deref
for crate::Zenject::DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder {
    type Target = crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder")]
impl std::ops::DerefMut
for crate::Zenject::DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder")]
impl crate::Zenject::DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder {
    pub fn New(
        subContainerBindInfo: *mut crate::Zenject::SubContainerCreatorBindInfo,
        bindInfo: *mut crate::Zenject::BindInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (subContainerBindInfo, bindInfo))?;
        Ok(__cordl_object)
    }
    pub fn WithDefaultGameObjectParent(
        &mut self,
        defaultParentName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("WithDefaultGameObjectParent", (defaultParentName))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        subContainerBindInfo: *mut crate::Zenject::SubContainerCreatorBindInfo,
        bindInfo: *mut crate::Zenject::BindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (subContainerBindInfo, bindInfo))?;
        Ok(__cordl_ret)
    }
    pub fn get_SubContainerCreatorBindInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::SubContainerCreatorBindInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::SubContainerCreatorBindInfo = __cordl_object
            .invoke("get_SubContainerCreatorBindInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_SubContainerCreatorBindInfo(
        &mut self,
        value: *mut crate::Zenject::SubContainerCreatorBindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SubContainerCreatorBindInfo", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::DefaultParentScopeConcreteIdArgConditionCopyNonLazyBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
