#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Tls+DefaultTlsAgreementCredentials")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultTlsAgreementCredentials {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsAgreementCredentials,
    pub mCertificate: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::Certificate>,
    pub mPrivateKey:
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter>,
    pub mBasicAgreement:
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IBasicAgreement>,
    pub mTruncateAgreement: bool,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Tls+DefaultTlsAgreementCredentials")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsAgreementCredentials
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Tls";
    const CLASS_NAME: &'static str = "DefaultTlsAgreementCredentials";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsAgreementCredentials")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsAgreementCredentials {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsAgreementCredentials;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsAgreementCredentials")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsAgreementCredentials {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsAgreementCredentials")]
impl crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsAgreementCredentials {
    pub fn GenerateAgreement(
        &mut self,
        peerPublicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                    >), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>, 1usize>(
                        "GenerateAgreement",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateAgreement",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>> =
            unsafe { cordl_method_info.invoke_unchecked(self, (peerPublicKey))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        certificate: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::Certificate>,
        privateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certificate, privateKey))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::Certificate>,
        privateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Tls::Certificate,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (certificate, privateKey))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Certificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::Certificate>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Tls::Certificate,
                        >,
                        0usize,
                    >("get_Certificate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Certificate", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Tls+DefaultTlsAgreementCredentials")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsAgreementCredentials
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
