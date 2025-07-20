#[cfg(feature = "System+Net+AuthenticationManager")]
#[repr(C)]
#[derive(Debug)]
pub struct AuthenticationManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+AuthenticationManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::AuthenticationManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "AuthenticationManager";
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
#[cfg(feature = "System+Net+AuthenticationManager")]
impl std::ops::Deref for crate::System::Net::AuthenticationManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+AuthenticationManager")]
impl std::ops::DerefMut for crate::System::Net::AuthenticationManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+AuthenticationManager")]
impl crate::System::Net::AuthenticationManager {
    pub fn Authenticate(
        challenge: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
        credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Authorization>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
                            quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Net::Authorization>,
                        3usize,
                    >("Authenticate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Authenticate", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::Authorization> = unsafe {
            method.invoke_unchecked((), (challenge, request, credentials))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoAuthenticate(
        challenge: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
        credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Authorization>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
                            quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Net::Authorization>,
                        3usize,
                    >("DoAuthenticate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "DoAuthenticate", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::Authorization> = unsafe {
            method.invoke_unchecked((), (challenge, request, credentials))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureModules() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("EnsureModules")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EnsureModules", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PreAuthenticate(
        request: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
        credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Authorization>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
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
                            Self::class(), "PreAuthenticate", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::Authorization> = unsafe {
            method.invoke_unchecked((), (request, credentials))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+AuthenticationManager")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::AuthenticationManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
