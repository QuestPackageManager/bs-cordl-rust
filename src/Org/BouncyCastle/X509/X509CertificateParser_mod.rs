#[cfg(feature = "Org+BouncyCastle+X509+X509CertificateParser")]
#[repr(C)]
#[derive(Debug)]
pub struct X509CertificateParser {
    __cordl_parent: crate::System::Object,
    pub sData: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    pub sDataObjectCount: i32,
    pub currentStream: *mut crate::System::IO::Stream,
}
#[cfg(feature = "Org+BouncyCastle+X509+X509CertificateParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::X509::X509CertificateParser
    => "Org.BouncyCastle.X509"."X509CertificateParser"
);
#[cfg(feature = "Org+BouncyCastle+X509+X509CertificateParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::X509::X509CertificateParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509CertificateParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::X509::X509CertificateParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509CertificateParser")]
impl crate::Org::BouncyCastle::X509::X509CertificateParser {
    pub fn ReadCertificate_Il2CppArray0(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509Certificate = __cordl_object
            .invoke("ReadCertificate", (input))?;
        Ok(__cordl_ret)
    }
    pub fn ReadCertificate_Stream1(
        &mut self,
        inStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509Certificate = __cordl_object
            .invoke("ReadCertificate", (inStream))?;
        Ok(__cordl_ret)
    }
    pub fn ReadCertificates_Il2CppArray0(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("ReadCertificates", (input))?;
        Ok(__cordl_ret)
    }
    pub fn ReadCertificates_Stream1(
        &mut self,
        inStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("ReadCertificates", (inStream))?;
        Ok(__cordl_ret)
    }
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
    pub fn ReadPemCertificate(
        &mut self,
        inStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509Certificate = __cordl_object
            .invoke("ReadPemCertificate", (inStream))?;
        Ok(__cordl_ret)
    }
    pub fn ReadDerCertificate(
        &mut self,
        dIn: *mut crate::Org::BouncyCastle::Asn1::Asn1InputStream,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509Certificate = __cordl_object
            .invoke("ReadDerCertificate", (dIn))?;
        Ok(__cordl_ret)
    }
    pub fn CreateX509Certificate(
        &mut self,
        c: *mut crate::Org::BouncyCastle::Asn1::X509::X509CertificateStructure,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509Certificate = __cordl_object
            .invoke("CreateX509Certificate", (c))?;
        Ok(__cordl_ret)
    }
    pub fn GetCertificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509Certificate = __cordl_object
            .invoke("GetCertificate", ())?;
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
#[cfg(feature = "Org+BouncyCastle+X509+X509CertificateParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::X509::X509CertificateParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
