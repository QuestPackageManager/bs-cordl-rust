#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevRepContentBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct RevRepContentBuilder {
    __cordl_parent: crate::System::Object,
    pub status: *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
    pub revCerts: *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
    pub crls: *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevRepContentBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder => "Org.BouncyCastle.Asn1.Cmp"
    ."RevRepContentBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevRepContentBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevRepContentBuilder")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevRepContentBuilder")]
impl crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder {
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
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::RevRepContent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::RevRepContent = __cordl_object
            .invoke("Build", ())?;
        Ok(__cordl_ret)
    }
    pub fn Add_PkiStatusInfo0(
        &mut self,
        status: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder = __cordl_object
            .invoke("Add", (status))?;
        Ok(__cordl_ret)
    }
    pub fn Add_CertId1(
        &mut self,
        status: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
        certId: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertId,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder = __cordl_object
            .invoke("Add", (status, certId))?;
        Ok(__cordl_ret)
    }
    pub fn AddCrl(
        &mut self,
        crl: *mut crate::Org::BouncyCastle::Asn1::X509::CertificateList,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder = __cordl_object
            .invoke("AddCrl", (crl))?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevRepContentBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
