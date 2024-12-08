#[cfg(feature = "Org+BouncyCastle+X509+X509CertPairParser")]
#[repr(C)]
#[derive(Debug)]
pub struct X509CertPairParser {
    __cordl_parent: crate::System::Object,
    pub currentStream: *mut crate::System::IO::Stream,
}
#[cfg(feature = "Org+BouncyCastle+X509+X509CertPairParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::X509::X509CertPairParser =>
    "Org.BouncyCastle.X509"."X509CertPairParser"
);
#[cfg(feature = "Org+BouncyCastle+X509+X509CertPairParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::X509::X509CertPairParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509CertPairParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::X509::X509CertPairParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509CertPairParser")]
impl crate::Org::BouncyCastle::X509::X509CertPairParser {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadCertPairs_Il2CppArray0(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("ReadCertPairs", (input))?;
        Ok(__cordl_ret)
    }
    pub fn ReadCertPairs_Stream1(
        &mut self,
        inStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("ReadCertPairs", (inStream))?;
        Ok(__cordl_ret)
    }
    pub fn ReadDerCrossCertificatePair(
        &mut self,
        inStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::X509CertificatePair,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509CertificatePair = __cordl_object
            .invoke("ReadDerCrossCertificatePair", (inStream))?;
        Ok(__cordl_ret)
    }
    pub fn ReadCertPair_Il2CppArray0(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::X509CertificatePair,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509CertificatePair = __cordl_object
            .invoke("ReadCertPair", (input))?;
        Ok(__cordl_ret)
    }
    pub fn ReadCertPair_Stream1(
        &mut self,
        inStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::X509CertificatePair,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509CertificatePair = __cordl_object
            .invoke("ReadCertPair", (inStream))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509CertPairParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::X509::X509CertPairParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
