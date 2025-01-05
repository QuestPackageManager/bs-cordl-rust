#[cfg(feature = "Org+BouncyCastle+X509+X509CrlParser")]
#[repr(C)]
#[derive(Debug)]
pub struct X509CrlParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub lazyAsn1: bool,
    pub sCrlData: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Set>,
    pub sCrlDataObjectCount: i32,
    pub currentCrlStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
}
#[cfg(feature = "Org+BouncyCastle+X509+X509CrlParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::X509::X509CrlParser =>
    "Org.BouncyCastle.X509"."X509CrlParser"
);
#[cfg(feature = "Org+BouncyCastle+X509+X509CrlParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::X509::X509CrlParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509CrlParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::X509::X509CrlParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509CrlParser")]
impl crate::Org::BouncyCastle::X509::X509CrlParser {
    pub fn CreateX509Crl(
        &mut self,
        c: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::CertificateList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Crl,
        > = __cordl_object.invoke("CreateX509Crl", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCrl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Crl,
        > = __cordl_object.invoke("GetCrl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool1(
        lazyAsn1: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lazyAsn1))?;
        Ok(__cordl_object.into())
    }
    pub fn ReadCrl_Il2CppArray0(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Crl,
        > = __cordl_object.invoke("ReadCrl", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadCrl_Stream1(
        &mut self,
        inStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Crl,
        > = __cordl_object.invoke("ReadCrl", (inStream))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadCrls_Il2CppArray0(
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
        > = __cordl_object.invoke("ReadCrls", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadCrls_Stream1(
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
        > = __cordl_object.invoke("ReadCrls", (inStream))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadDerCrl(
        &mut self,
        dIn: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1InputStream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Crl,
        > = __cordl_object.invoke("ReadDerCrl", (dIn))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadPemCrl(
        &mut self,
        inStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Crl,
        > = __cordl_object.invoke("ReadPemCrl", (inStream))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        lazyAsn1: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lazyAsn1))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509CrlParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::X509::X509CrlParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
