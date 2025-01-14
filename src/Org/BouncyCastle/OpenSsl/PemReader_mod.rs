#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemReader")]
#[repr(C)]
#[derive(Debug)]
pub struct PemReader {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::Pem::PemReader,
    pub pFinder: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::OpenSsl::IPasswordFinder,
    >,
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemReader")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::OpenSsl::PemReader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.OpenSsl";
    const CLASS_NAME: &'static str = "PemReader";
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
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemReader")]
impl std::ops::Deref for crate::Org::BouncyCastle::OpenSsl::PemReader {
    type Target = crate::Org::BouncyCastle::Utilities::IO::Pem::PemReader;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemReader")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::OpenSsl::PemReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemReader")]
impl crate::Org::BouncyCastle::OpenSsl::PemReader {
    pub fn GetCurveParameters(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
                >,
                1usize,
            >("GetCurveParameters")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCurveParameters", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = unsafe { method.invoke_unchecked((), (name)) };
        Ok(__cordl_ret.into())
    }
    pub fn New_IPasswordFinder1(
        reader: quest_hook::libil2cpp::Gc<crate::System::IO::TextReader>,
        pFinder: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::OpenSsl::IPasswordFinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reader, pFinder))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextReader0(
        reader: quest_hook::libil2cpp::Gc<crate::System::IO::TextReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reader))?;
        Ok(__cordl_object.into())
    }
    pub fn ReadAttributeCertificate(
        &mut self,
        pemObject: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
                >,
                1usize,
            >("ReadAttributeCertificate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadAttributeCertificate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        > = unsafe { method.invoke_unchecked(self, (pemObject)) };
        Ok(__cordl_ret.into())
    }
    pub fn ReadCertificate(
        &mut self,
        pemObject: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::X509::X509Certificate,
                >,
                1usize,
            >("ReadCertificate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadCertificate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        > = unsafe { method.invoke_unchecked(self, (pemObject)) };
        Ok(__cordl_ret.into())
    }
    pub fn ReadCertificateRequest(
        &mut self,
        pemObject: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkcs::Pkcs10CertificationRequest,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Pkcs::Pkcs10CertificationRequest,
                >,
                1usize,
            >("ReadCertificateRequest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadCertificateRequest", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkcs::Pkcs10CertificationRequest,
        > = unsafe { method.invoke_unchecked(self, (pemObject)) };
        Ok(__cordl_ret.into())
    }
    pub fn ReadCrl(
        &mut self,
        pemObject: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
                >),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
                1usize,
            >("ReadCrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadCrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Crl,
        > = unsafe { method.invoke_unchecked(self, (pemObject)) };
        Ok(__cordl_ret.into())
    }
    pub fn ReadObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("ReadObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadObject", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPkcs7(
        &mut self,
        pemObject: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::ContentInfo>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
                >,
                1usize,
            >("ReadPkcs7")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadPkcs7", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
        > = unsafe { method.invoke_unchecked(self, (pemObject)) };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPrivateKey(
        &mut self,
        pemObject: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
                >),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                1usize,
            >("ReadPrivateKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadPrivateKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (pemObject)) };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPublicKey(
        &mut self,
        pemObject: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                >,
                1usize,
            >("ReadPublicKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadPublicKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = unsafe { method.invoke_unchecked(self, (pemObject)) };
        Ok(__cordl_ret.into())
    }
    pub fn ReadRsaPublicKey(
        &mut self,
        pemObject: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                >,
                1usize,
            >("ReadRsaPublicKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadRsaPublicKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = unsafe { method.invoke_unchecked(self, (pemObject)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IPasswordFinder1(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::IO::TextReader>,
        pFinder: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::OpenSsl::IPasswordFinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::IO::TextReader>,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::OpenSsl::IPasswordFinder,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (reader, pFinder))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextReader0(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::IO::TextReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IO::TextReader>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (reader))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemReader")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::OpenSsl::PemReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
