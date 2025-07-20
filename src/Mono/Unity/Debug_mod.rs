#[cfg(feature = "Mono+Unity+Debug")]
#[repr(C)]
#[derive(Debug)]
pub struct Debug {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Unity+Debug")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::Unity::Debug {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "Debug";
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
#[cfg(feature = "Mono+Unity+Debug")]
impl std::ops::Deref for crate::Mono::Unity::Debug {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+Debug")]
impl std::ops::DerefMut for crate::Mono::Unity::Debug {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+Debug")]
impl crate::Mono::Unity::Debug {
    pub fn CheckAndThrow_Il2CppString_AlertDescription0(
        errorState: crate::Mono::Unity::UnityTls_unitytls_errorstate,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultAlert: crate::Mono::Security::Interface::AlertDescription,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Unity::Debug as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Mono::Unity::UnityTls_unitytls_errorstate,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::Mono::Security::Interface::AlertDescription,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("CheckAndThrow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Unity::Debug as quest_hook::libil2cpp::Type >
                    ::class(), "CheckAndThrow", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (errorState, context, defaultAlert))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckAndThrow_UnityTls_unitytls_x509verify_result_Il2CppString_AlertDescription1(
        errorState: crate::Mono::Unity::UnityTls_unitytls_errorstate,
        verifyResult: crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultAlert: crate::Mono::Security::Interface::AlertDescription,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Unity::Debug as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Mono::Unity::UnityTls_unitytls_errorstate,
                    crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::Mono::Security::Interface::AlertDescription,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("CheckAndThrow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Unity::Debug as quest_hook::libil2cpp::Type >
                    ::class(), "CheckAndThrow", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked((), (errorState, verifyResult, context, defaultAlert))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+Debug")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Unity::Debug {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
