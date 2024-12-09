#[cfg(feature = "Zenject+InstantiateCallbackConditionCopyNonLazyBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct InstantiateCallbackConditionCopyNonLazyBinder {
    __cordl_parent: crate::Zenject::ConditionCopyNonLazyBinder,
}
#[cfg(feature = "Zenject+InstantiateCallbackConditionCopyNonLazyBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::InstantiateCallbackConditionCopyNonLazyBinder => "Zenject"
    ."InstantiateCallbackConditionCopyNonLazyBinder"
);
#[cfg(feature = "Zenject+InstantiateCallbackConditionCopyNonLazyBinder")]
impl std::ops::Deref for crate::Zenject::InstantiateCallbackConditionCopyNonLazyBinder {
    type Target = crate::Zenject::ConditionCopyNonLazyBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InstantiateCallbackConditionCopyNonLazyBinder")]
impl std::ops::DerefMut
for crate::Zenject::InstantiateCallbackConditionCopyNonLazyBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InstantiateCallbackConditionCopyNonLazyBinder")]
impl crate::Zenject::InstantiateCallbackConditionCopyNonLazyBinder {
    #[cfg(
        feature = "Zenject+InstantiateCallbackConditionCopyNonLazyBinder+__c__DisplayClass2_0_1"
    )]
    pub type __c__DisplayClass2_0_1<T: quest_hook::libil2cpp::Type> = crate::Zenject::InstantiateCallbackConditionCopyNonLazyBinder___c__DisplayClass2_0_1<
        T,
    >;
    pub fn New(
        bindInfo: *mut crate::Zenject::BindInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo))?;
        Ok(__cordl_object)
    }
    pub fn OnInstantiated_Action_2_0(
        &mut self,
        callback: *mut crate::System::Action_2<
            *mut crate::Zenject::InjectContext,
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("OnInstantiated", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn OnInstantiated_Action_2_1<T>(
        &mut self,
        callback: *mut crate::System::Action_2<*mut crate::Zenject::InjectContext, T>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConditionCopyNonLazyBinder>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConditionCopyNonLazyBinder = __cordl_object
            .invoke("OnInstantiated", (callback))?;
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
}
#[cfg(feature = "Zenject+InstantiateCallbackConditionCopyNonLazyBinder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::InstantiateCallbackConditionCopyNonLazyBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
