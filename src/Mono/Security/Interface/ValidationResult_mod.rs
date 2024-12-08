#[cfg(feature = "Mono+Security+Interface+ValidationResult")]
#[repr(C)]
#[derive(Debug)]
pub struct ValidationResult {
    __cordl_parent: crate::System::Object,
    pub trusted: bool,
    pub user_denied: bool,
    pub error_code: i32,
    pub policy_errors: crate::System::Nullable_1<
        crate::Mono::Security::Interface::MonoSslPolicyErrors,
    >,
}
#[cfg(feature = "Mono+Security+Interface+ValidationResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Interface::ValidationResult =>
    "Mono.Security.Interface"."ValidationResult"
);
#[cfg(feature = "Mono+Security+Interface+ValidationResult")]
impl std::ops::Deref for crate::Mono::Security::Interface::ValidationResult {
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
        trusted: bool,
        user_denied: bool,
        error_code: i32,
        policy_errors: crate::System::Nullable_1<
            crate::Mono::Security::Interface::MonoSslPolicyErrors,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (trusted, user_denied, error_code, policy_errors))?;
        Ok(__cordl_ret)
    }
    pub fn get_Trusted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Trusted", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UserDenied(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UserDenied", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        trusted: bool,
        user_denied: bool,
        error_code: i32,
        policy_errors: crate::System::Nullable_1<
            crate::Mono::Security::Interface::MonoSslPolicyErrors,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (trusted, user_denied, error_code, policy_errors))?;
        Ok(__cordl_object)
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
