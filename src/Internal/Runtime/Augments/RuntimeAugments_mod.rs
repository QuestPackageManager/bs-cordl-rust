#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeAugments {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Internal::Runtime::Augments::RuntimeAugments {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Internal.Runtime.Augments";
    const CLASS_NAME: &'static str = "RuntimeAugments";
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
#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
impl std::ops::Deref for crate::Internal::Runtime::Augments::RuntimeAugments {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
impl std::ops::DerefMut for crate::Internal::Runtime::Augments::RuntimeAugments {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
impl crate::Internal::Runtime::Augments::RuntimeAugments {
    pub fn ReportUnhandledException(
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Exception>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ReportUnhandledException")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReportUnhandledException", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (exception))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Callbacks() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Internal::Runtime::Augments::ReflectionExecutionDomainCallbacks,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Internal::Runtime::Augments::ReflectionExecutionDomainCallbacks,
                        >,
                        0usize,
                    >("get_Callbacks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Callbacks", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Internal::Runtime::Augments::ReflectionExecutionDomainCallbacks,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
impl quest_hook::libil2cpp::ObjectType
for crate::Internal::Runtime::Augments::RuntimeAugments {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
