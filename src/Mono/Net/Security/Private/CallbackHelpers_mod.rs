#[cfg(feature = "Mono+Net+Security+Private+CallbackHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct CallbackHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Net+Security+Private+CallbackHelpers")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Net::Security::Private::CallbackHelpers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Net.Security.Private";
    const CLASS_NAME: &'static str = "CallbackHelpers";
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
#[cfg(feature = "Mono+Net+Security+Private+CallbackHelpers")]
impl std::ops::Deref for crate::Mono::Net::Security::Private::CallbackHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+Private+CallbackHelpers")]
impl std::ops::DerefMut for crate::Mono::Net::Security::Private::CallbackHelpers {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+Private+CallbackHelpers")]
impl crate::Mono::Net::Security::Private::CallbackHelpers {
    pub fn MonoToInternal(
        callback: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoLocalCertificateSelectionCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::LocalCertSelectionCallback,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Mono::Security::Interface::MonoLocalCertificateSelectionCallback,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Net::Security::LocalCertSelectionCallback,
                        >,
                        1usize,
                    >("MonoToInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MonoToInternal", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::LocalCertSelectionCallback,
        > = unsafe { cordl_method_info.invoke_unchecked((), (callback))? };
        Ok(__cordl_ret.into())
    }
    pub fn PublicToMono(
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::RemoteCertificateValidationCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoRemoteCertificateValidationCallback,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Net::Security::RemoteCertificateValidationCallback,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Mono::Security::Interface::MonoRemoteCertificateValidationCallback,
                        >,
                        1usize,
                    >("PublicToMono")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PublicToMono", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoRemoteCertificateValidationCallback,
        > = unsafe { cordl_method_info.invoke_unchecked((), (callback))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Net+Security+Private+CallbackHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Net::Security::Private::CallbackHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
