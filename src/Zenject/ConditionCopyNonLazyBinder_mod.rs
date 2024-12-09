#[cfg(feature = "Zenject+ConditionCopyNonLazyBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct ConditionCopyNonLazyBinder {
    __cordl_parent: crate::Zenject::CopyNonLazyBinder,
}
#[cfg(feature = "Zenject+ConditionCopyNonLazyBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ConditionCopyNonLazyBinder => "Zenject"
    ."ConditionCopyNonLazyBinder"
);
#[cfg(feature = "Zenject+ConditionCopyNonLazyBinder")]
impl std::ops::Deref for crate::Zenject::ConditionCopyNonLazyBinder {
    type Target = crate::Zenject::CopyNonLazyBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConditionCopyNonLazyBinder")]
impl std::ops::DerefMut for crate::Zenject::ConditionCopyNonLazyBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConditionCopyNonLazyBinder")]
impl crate::Zenject::ConditionCopyNonLazyBinder {
    #[cfg(feature = "Zenject+ConditionCopyNonLazyBinder+__c__4_1")]
    pub type __c__4_1<T: quest_hook::libil2cpp::Type> = crate::Zenject::ConditionCopyNonLazyBinder___c__4_1<
        T,
    >;
    #[cfg(feature = "Zenject+ConditionCopyNonLazyBinder+__c__5_1")]
    pub type __c__5_1<T: quest_hook::libil2cpp::Type> = crate::Zenject::ConditionCopyNonLazyBinder___c__5_1<
        T,
    >;
    #[cfg(feature = "Zenject+ConditionCopyNonLazyBinder+__c__DisplayClass2_0")]
    pub type __c__DisplayClass2_0 = crate::Zenject::ConditionCopyNonLazyBinder___c__DisplayClass2_0;
    #[cfg(feature = "Zenject+ConditionCopyNonLazyBinder+__c__DisplayClass3_0")]
    pub type __c__DisplayClass3_0 = crate::Zenject::ConditionCopyNonLazyBinder___c__DisplayClass3_0;
    #[cfg(feature = "Zenject+ConditionCopyNonLazyBinder+__c__DisplayClass3_1")]
    pub type __c__DisplayClass3_1 = crate::Zenject::ConditionCopyNonLazyBinder___c__DisplayClass3_1;
    pub fn New(
        bindInfo: *mut crate::Zenject::BindInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo))?;
        Ok(__cordl_object)
    }
    pub fn When(
        &mut self,
        condition: *mut crate::Zenject::BindingCondition,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("When", (condition))?;
        Ok(__cordl_ret)
    }
    pub fn WhenInjectedIntoInstance(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("WhenInjectedIntoInstance", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn WhenInjectedInto_1<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("WhenInjectedInto", ())?;
        Ok(__cordl_ret)
    }
    pub fn WhenInjectedInto_Il2CppArray0(
        &mut self,
        targets: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("WhenInjectedInto", (targets))?;
        Ok(__cordl_ret)
    }
    pub fn WhenNotInjectedInto<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::CopyNonLazyBinder>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::CopyNonLazyBinder = __cordl_object
            .invoke("WhenNotInjectedInto", ())?;
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
#[cfg(feature = "Zenject+ConditionCopyNonLazyBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ConditionCopyNonLazyBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
