#[cfg(feature = "cordl_class_Zenject+ConditionCopyNonLazyBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct ConditionCopyNonLazyBinder {
    __cordl_parent: crate::Zenject::CopyNonLazyBinder,
}
#[cfg(feature = "cordl_class_Zenject+ConditionCopyNonLazyBinder")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::ConditionCopyNonLazyBinder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "ConditionCopyNonLazyBinder";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Zenject+ConditionCopyNonLazyBinder")]
impl std::ops::Deref for crate::Zenject::ConditionCopyNonLazyBinder {
    type Target = crate::Zenject::CopyNonLazyBinder;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConditionCopyNonLazyBinder")]
impl std::ops::DerefMut for crate::Zenject::ConditionCopyNonLazyBinder {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConditionCopyNonLazyBinder")]
impl crate::Zenject::ConditionCopyNonLazyBinder {
    pub fn New(
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn When(
        &mut self,
        condition: quest_hook::libil2cpp::Gc<crate::Zenject::BindingCondition>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::Zenject::CopyNonLazyBinder>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::Zenject::BindingCondition>),
                        quest_hook::libil2cpp::Gc<crate::Zenject::CopyNonLazyBinder>,
                        1usize,
                    >("When")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "When",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::CopyNonLazyBinder> =
            unsafe { cordl_method_info.invoke_unchecked(self, (condition))? };
        Ok(__cordl_ret.into())
    }
    pub fn WhenInjectedIntoInstance(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::Zenject::CopyNonLazyBinder>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Gc<crate::Zenject::CopyNonLazyBinder>,
                        1usize,
                    >("WhenInjectedIntoInstance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WhenInjectedIntoInstance", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::CopyNonLazyBinder> =
            unsafe { cordl_method_info.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn WhenInjectedInto_1<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::Zenject::CopyNonLazyBinder>>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::Zenject::CopyNonLazyBinder>,
                        0usize,
                    >("WhenInjectedInto")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WhenInjectedInto", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::CopyNonLazyBinder> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn WhenInjectedInto_Il2CppArray0(
        &mut self,
        targets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<quest_hook::libil2cpp::Gc<crate::System::Type>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::Zenject::CopyNonLazyBinder>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        >,
                    >), quest_hook::libil2cpp::Gc<crate::Zenject::CopyNonLazyBinder>, 1usize>(
                        "WhenInjectedInto",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WhenInjectedInto",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::CopyNonLazyBinder> =
            unsafe { cordl_method_info.invoke_unchecked(self, (targets))? };
        Ok(__cordl_ret.into())
    }
    pub fn WhenNotInjectedInto<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::Zenject::CopyNonLazyBinder>>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::Zenject::CopyNonLazyBinder>,
                        0usize,
                    >("WhenNotInjectedInto")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WhenNotInjectedInto", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::CopyNonLazyBinder> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (bindInfo))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Zenject+ConditionCopyNonLazyBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ConditionCopyNonLazyBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
