#[cfg(feature = "Zenject+ScopeConcreteIdArgConditionCopyNonLazyBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct ScopeConcreteIdArgConditionCopyNonLazyBinder {
    __cordl_parent: crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder,
}
#[cfg(feature = "Zenject+ScopeConcreteIdArgConditionCopyNonLazyBinder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "ScopeConcreteIdArgConditionCopyNonLazyBinder";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("AsCached", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AsSingle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("AsSingle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AsTransient(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("AsTransient", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindInfo))?;
        Ok(__cordl_ret.into())
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
