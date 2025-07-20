#[cfg(feature = "Mono+Security+Interface+ValidationResult")]
#[repr(C)]
#[derive(Debug)]
pub struct ValidationResult {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub trusted: bool,
    pub user_denied: bool,
    pub error_code: i32,
    pub policy_errors: crate::System::Nullable_1<
        crate::Mono::Security::Interface::MonoSslPolicyErrors,
    >,
}
#[cfg(feature = "Mono+Security+Interface+ValidationResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Security::Interface::ValidationResult {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Security.Interface";
    const CLASS_NAME: &'static str = "ValidationResult";
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
#[cfg(feature = "Mono+Security+Interface+ValidationResult")]
impl std::ops::Deref for crate::Mono::Security::Interface::ValidationResult {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Interface+ValidationResult")]
impl std::ops::DerefMut for crate::Mono::Security::Interface::ValidationResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Interface+ValidationResult")]
impl crate::Mono::Security::Interface::ValidationResult {
    pub fn New(
        trusted: bool,
        user_denied: bool,
        error_code: i32,
        policy_errors: crate::System::Nullable_1<
            crate::Mono::Security::Interface::MonoSslPolicyErrors,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (trusted, user_denied, error_code, policy_errors))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        trusted: bool,
        user_denied: bool,
        error_code: i32,
        policy_errors: crate::System::Nullable_1<
            crate::Mono::Security::Interface::MonoSslPolicyErrors,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Security::Interface::ValidationResult as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    bool,
                    bool,
                    i32,
                    crate::System::Nullable_1<
                        crate::Mono::Security::Interface::MonoSslPolicyErrors,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Security::Interface::ValidationResult as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (trusted, user_denied, error_code, policy_errors),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Trusted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Security::Interface::ValidationResult as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_Trusted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Security::Interface::ValidationResult as
                    quest_hook::libil2cpp::Type > ::class(), "get_Trusted", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_UserDenied(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Security::Interface::ValidationResult as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_UserDenied")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Security::Interface::ValidationResult as
                    quest_hook::libil2cpp::Type > ::class(), "get_UserDenied", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Security+Interface+ValidationResult")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Interface::ValidationResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
