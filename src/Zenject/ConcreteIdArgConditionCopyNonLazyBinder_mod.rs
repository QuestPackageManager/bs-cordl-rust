#[cfg(feature = "Zenject+ConcreteIdArgConditionCopyNonLazyBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct ConcreteIdArgConditionCopyNonLazyBinder {
    __cordl_parent: crate::Zenject::ArgConditionCopyNonLazyBinder,
}
#[cfg(feature = "Zenject+ConcreteIdArgConditionCopyNonLazyBinder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "ConcreteIdArgConditionCopyNonLazyBinder";
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
#[cfg(feature = "Zenject+ConcreteIdArgConditionCopyNonLazyBinder")]
impl std::ops::Deref for crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder {
    type Target = crate::Zenject::ArgConditionCopyNonLazyBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConcreteIdArgConditionCopyNonLazyBinder")]
impl std::ops::DerefMut for crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConcreteIdArgConditionCopyNonLazyBinder")]
impl crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder {
    pub fn New(
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn WithConcreteId(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
                1usize,
            >("WithConcreteId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WithConcreteId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked(self, (id)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bindInfo))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+ConcreteIdArgConditionCopyNonLazyBinder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::ConcreteIdArgConditionCopyNonLazyBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
