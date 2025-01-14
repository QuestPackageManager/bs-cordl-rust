#[cfg(feature = "System+Net+NtlmClient")]
#[repr(C)]
#[derive(Debug)]
pub struct NtlmClient {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub authObject: quest_hook::libil2cpp::Gc<crate::System::Net::IAuthenticationModule>,
}
#[cfg(feature = "System+Net+NtlmClient")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::NtlmClient {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "NtlmClient";
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
#[cfg(feature = "System+Net+NtlmClient")]
impl std::ops::Deref for crate::System::Net::NtlmClient {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NtlmClient")]
impl std::ops::DerefMut for crate::System::Net::NtlmClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NtlmClient")]
impl crate::System::Net::NtlmClient {
    pub fn Authenticate(
        &mut self,
        challenge: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        webRequest: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
        credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Authorization>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
                    quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Net::Authorization>,
                3usize,
            >("Authenticate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Authenticate", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::Authorization> = unsafe {
            method.invoke_unchecked(self, (challenge, webRequest, credentials))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PreAuthenticate(
        &mut self,
        webRequest: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
        credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Authorization>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
                    quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Net::Authorization>,
                2usize,
            >("PreAuthenticate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PreAuthenticate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::Authorization> = unsafe {
            method.invoke_unchecked(self, (webRequest, credentials))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_AuthenticationType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_AuthenticationType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_AuthenticationType", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+NtlmClient")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::NtlmClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+NtlmClient")]
impl AsRef<crate::System::Net::IAuthenticationModule>
for crate::System::Net::NtlmClient {
    fn as_ref(&self) -> &crate::System::Net::IAuthenticationModule {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+NtlmClient")]
impl AsMut<crate::System::Net::IAuthenticationModule>
for crate::System::Net::NtlmClient {
    fn as_mut(&mut self) -> &mut crate::System::Net::IAuthenticationModule {
        unsafe { std::mem::transmute(self) }
    }
}
