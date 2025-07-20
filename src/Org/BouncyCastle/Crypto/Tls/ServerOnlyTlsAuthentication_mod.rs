#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerOnlyTlsAuthentication")]
#[repr(C)]
#[derive(Debug)]
pub struct ServerOnlyTlsAuthentication {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerOnlyTlsAuthentication")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Tls::ServerOnlyTlsAuthentication {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Tls";
    const CLASS_NAME: &'static str = "ServerOnlyTlsAuthentication";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerOnlyTlsAuthentication")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::ServerOnlyTlsAuthentication {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerOnlyTlsAuthentication")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::ServerOnlyTlsAuthentication {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerOnlyTlsAuthentication")]
impl crate::Org::BouncyCastle::Crypto::Tls::ServerOnlyTlsAuthentication {
    pub fn GetClientCredentials(
        &mut self,
        certificateRequest: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials,
                        >,
                        1usize,
                    >("GetClientCredentials")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetClientCredentials", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (certificateRequest))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyServerCertificate(
        &mut self,
        serverCertificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Tls::Certificate,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("NotifyServerCertificate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NotifyServerCertificate", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (serverCertificate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerOnlyTlsAuthentication")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::ServerOnlyTlsAuthentication {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerOnlyTlsAuthentication")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Tls::TlsAuthentication>
for crate::Org::BouncyCastle::Crypto::Tls::ServerOnlyTlsAuthentication {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Tls::TlsAuthentication {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ServerOnlyTlsAuthentication")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Tls::TlsAuthentication>
for crate::Org::BouncyCastle::Crypto::Tls::ServerOnlyTlsAuthentication {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::Tls::TlsAuthentication {
        unsafe { std::mem::transmute(self) }
    }
}
