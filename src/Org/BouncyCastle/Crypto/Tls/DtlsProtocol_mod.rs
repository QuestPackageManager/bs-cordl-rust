#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsProtocol")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsProtocol {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mSecureRandom: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Security::SecureRandom,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsProtocol")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::DtlsProtocol =>
    "Org.BouncyCastle.Crypto.Tls"."DtlsProtocol"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsProtocol")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::DtlsProtocol {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsProtocol")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::DtlsProtocol {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsProtocol")]
impl crate::Org::BouncyCastle::Crypto::Tls::DtlsProtocol {
    pub fn ApplyMaxFragmentLengthExtension(
        recordLayer: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
        >,
        maxFragmentLength: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ApplyMaxFragmentLengthExtension",
                (recordLayer, maxFragmentLength),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateMaxFragmentLengthExtension(
        resumedSession: bool,
        clientExtensions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IDictionary,
        >,
        serverExtensions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IDictionary,
        >,
        alertDescription: u8,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EvaluateMaxFragmentLengthExtension",
                (resumedSession, clientExtensions, serverExtensions, alertDescription),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCertificate(
        certificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateCertificate", (certificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateSupplementalData(
        supplementalData: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateSupplementalData", (supplementalData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (secureRandom))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessFinished(
        &mut self,
        body: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        expected_verify_data: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessFinished", (body, expected_verify_data))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateSelectedCipherSuite(
        selectedCipherSuite: i32,
        alertDescription: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ValidateSelectedCipherSuite",
                (selectedCipherSuite, alertDescription),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (secureRandom))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsProtocol")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DtlsProtocol {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
