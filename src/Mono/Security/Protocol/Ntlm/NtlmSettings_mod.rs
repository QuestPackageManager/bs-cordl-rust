#[cfg(feature = "cordl_class_Mono+Security+Protocol+Ntlm+NtlmSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct NtlmSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Mono+Security+Protocol+Ntlm+NtlmSettings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Security::Protocol::Ntlm::NtlmSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Security.Protocol.Ntlm";
    const CLASS_NAME: &'static str = "NtlmSettings";
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
#[cfg(feature = "cordl_class_Mono+Security+Protocol+Ntlm+NtlmSettings")]
impl std::ops::Deref for crate::Mono::Security::Protocol::Ntlm::NtlmSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_Mono+Security+Protocol+Ntlm+NtlmSettings")]
impl std::ops::DerefMut for crate::Mono::Security::Protocol::Ntlm::NtlmSettings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Protocol+Ntlm+NtlmSettings")]
impl crate::Mono::Security::Protocol::Ntlm::NtlmSettings {
    pub fn get_DefaultAuthLevel() -> quest_hook::libil2cpp::Result<
        crate::Mono::Security::Protocol::Ntlm::NtlmAuthLevel,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::Mono::Security::Protocol::Ntlm::NtlmAuthLevel,
                        0usize,
                    >("get_DefaultAuthLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_DefaultAuthLevel", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Security::Protocol::Ntlm::NtlmAuthLevel = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Mono+Security+Protocol+Ntlm+NtlmSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Protocol::Ntlm::NtlmSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
