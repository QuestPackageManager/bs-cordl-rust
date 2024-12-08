#[cfg(feature = "Zenject+ScopeConcreteIdArgConditionCopyNonLazyBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct ScopeConcreteIdArgConditionCopyNonLazyBinder {
    __cordl_parent: crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder,
}
#[cfg(feature = "Zenject+ScopeConcreteIdArgConditionCopyNonLazyBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder => "Zenject"
    ."ScopeConcreteIdArgConditionCopyNonLazyBinder"
);
#[cfg(feature = "Zenject+ScopeConcreteIdArgConditionCopyNonLazyBinder")]
impl std::ops::Deref for crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder {
    type Target = crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ScopeConcreteIdArgConditionCopyNonLazyBinder")]
impl std::ops::DerefMut
for crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ScopeConcreteIdArgConditionCopyNonLazyBinder")]
impl crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder {
    pub fn AsCached(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("AsCached", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bindInfo: *mut crate::Zenject::BindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindInfo))?;
        Ok(__cordl_ret)
    }
    pub fn AsSingle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("AsSingle", ())?;
        Ok(__cordl_ret)
    }
    pub fn AsTransient(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("AsTransient", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bindInfo: *mut crate::Zenject::BindInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+ScopeConcreteIdArgConditionCopyNonLazyBinder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
