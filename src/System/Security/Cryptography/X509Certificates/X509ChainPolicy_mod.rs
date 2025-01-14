#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainPolicy")]
#[repr(C)]
#[derive(Debug)]
pub struct X509ChainPolicy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub apps: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::OidCollection,
    >,
    pub cert: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::OidCollection,
    >,
    pub store: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
    >,
    pub store2: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    >,
    pub rflag: crate::System::Security::Cryptography::X509Certificates::X509RevocationFlag,
    pub mode: crate::System::Security::Cryptography::X509Certificates::X509RevocationMode,
    pub timeout: crate::System::TimeSpan,
    pub vflags: crate::System::Security::Cryptography::X509Certificates::X509VerificationFlags,
    pub vtime: crate::System::DateTime,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainPolicy")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.Cryptography.X509Certificates";
    const CLASS_NAME: &'static str = "X509ChainPolicy";
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
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainPolicy")]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainPolicy")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainPolicy")]
impl crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Reset", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
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
    pub fn get_ExtraStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
                >,
                0usize,
            >("get_ExtraStore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_ExtraStore", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_RevocationFlag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509RevocationFlag,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Security::Cryptography::X509Certificates::X509RevocationFlag,
                0usize,
            >("get_RevocationFlag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_RevocationFlag", 0usize
                )
            });
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509RevocationFlag = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_RevocationMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509RevocationMode,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Security::Cryptography::X509Certificates::X509RevocationMode,
                0usize,
            >("get_RevocationMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_RevocationMode", 0usize
                )
            });
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509RevocationMode = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_VerificationFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509VerificationFlags,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Security::Cryptography::X509Certificates::X509VerificationFlags,
                0usize,
            >("get_VerificationFlags")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_VerificationFlags", 0usize
                )
            });
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509VerificationFlags = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_VerificationTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::System::DateTime, 0usize>("get_VerificationTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_VerificationTime", 0usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_RevocationFlag(
        &mut self,
        value: crate::System::Security::Cryptography::X509Certificates::X509RevocationFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Security::Cryptography::X509Certificates::X509RevocationFlag),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_RevocationFlag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_RevocationFlag", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_RevocationMode(
        &mut self,
        value: crate::System::Security::Cryptography::X509Certificates::X509RevocationMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Security::Cryptography::X509Certificates::X509RevocationMode),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_RevocationMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_RevocationMode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_UrlRetrievalTimeout(
        &mut self,
        value: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::TimeSpan),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_UrlRetrievalTimeout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_UrlRetrievalTimeout", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_VerificationFlags(
        &mut self,
        value: crate::System::Security::Cryptography::X509Certificates::X509VerificationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Security::Cryptography::X509Certificates::X509VerificationFlags),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_VerificationFlags")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_VerificationFlags", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_VerificationTime(
        &mut self,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::DateTime),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_VerificationTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_VerificationTime", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainPolicy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
