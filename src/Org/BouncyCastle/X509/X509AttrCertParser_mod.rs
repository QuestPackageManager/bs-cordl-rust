#[cfg(feature = "Org+BouncyCastle+X509+X509AttrCertParser")]
#[repr(C)]
#[derive(Debug)]
pub struct X509AttrCertParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub sData: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Set>,
    pub sDataObjectCount: i32,
    pub currentStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
}
#[cfg(feature = "Org+BouncyCastle+X509+X509AttrCertParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::X509::X509AttrCertParser =>
    "Org.BouncyCastle.X509"."X509AttrCertParser"
);
#[cfg(feature = "Org+BouncyCastle+X509+X509AttrCertParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::X509::X509AttrCertParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509AttrCertParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::X509::X509AttrCertParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509AttrCertParser")]
impl crate::Org::BouncyCastle::X509::X509AttrCertParser {
    pub fn GetCertificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        > = __cordl_object.invoke("GetCertificate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadAttrCert_Il2CppArray0(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        > = __cordl_object.invoke("ReadAttrCert", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAttrCert_Stream1(
        &mut self,
        inStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        > = __cordl_object.invoke("ReadAttrCert", (inStream))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAttrCerts_Il2CppArray0(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = __cordl_object.invoke("ReadAttrCerts", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAttrCerts_Stream1(
        &mut self,
        inStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = __cordl_object.invoke("ReadAttrCerts", (inStream))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadDerCertificate(
        &mut self,
        dIn: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1InputStream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        > = __cordl_object.invoke("ReadDerCertificate", (dIn))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadPemCertificate(
        &mut self,
        inStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        > = __cordl_object.invoke("ReadPemCertificate", (inStream))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509AttrCertParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::X509::X509AttrCertParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
