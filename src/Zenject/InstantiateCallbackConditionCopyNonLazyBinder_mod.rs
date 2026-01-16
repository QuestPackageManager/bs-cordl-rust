#[cfg(feature = "cordl_class_Zenject+InstantiateCallbackConditionCopyNonLazyBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct InstantiateCallbackConditionCopyNonLazyBinder {
    __cordl_parent: crate::Zenject::ConditionCopyNonLazyBinder,
}
#[cfg(feature = "cordl_class_Zenject+InstantiateCallbackConditionCopyNonLazyBinder")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Zenject::InstantiateCallbackConditionCopyNonLazyBinder
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "InstantiateCallbackConditionCopyNonLazyBinder";
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
#[cfg(feature = "Zenject+InstantiateCallbackConditionCopyNonLazyBinder")]
impl std::ops::Deref for crate::Zenject::InstantiateCallbackConditionCopyNonLazyBinder {
    type Target = crate::Zenject::ConditionCopyNonLazyBinder;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InstantiateCallbackConditionCopyNonLazyBinder")]
impl std::ops::DerefMut for crate::Zenject::InstantiateCallbackConditionCopyNonLazyBinder {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InstantiateCallbackConditionCopyNonLazyBinder")]
impl crate::Zenject::InstantiateCallbackConditionCopyNonLazyBinder {
    pub fn New(
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn OnInstantiated_Action_2_0(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConditionCopyNonLazyBinder>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConditionCopyNonLazyBinder,
                        >,
                        1usize,
                    >("OnInstantiated")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnInstantiated", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::ConditionCopyNonLazyBinder> =
            unsafe { cordl_method_info.invoke_unchecked(self, (callback))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnInstantiated_Action_2_1<T>(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>, T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConditionCopyNonLazyBinder>,
    >
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
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                                T,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConditionCopyNonLazyBinder,
                        >,
                        1usize,
                    >("OnInstantiated")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnInstantiated", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::ConditionCopyNonLazyBinder> =
            unsafe { cordl_method_info.invoke_unchecked(self, (callback))? };
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
#[cfg(feature = "cordl_class_Zenject+InstantiateCallbackConditionCopyNonLazyBinder")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Zenject::InstantiateCallbackConditionCopyNonLazyBinder
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
