#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct PkiMessage {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub header: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
    pub body: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiBody,
    pub protection: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    pub extraCerts: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiMessage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::PkiMessage =>
    "Org.BouncyCastle.Asn1.Cmp"."PkiMessage"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiMessage")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiMessage")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiMessage")]
impl crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage {
    pub fn GetExtraCerts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
        > = __cordl_object.invoke("GetExtraCerts", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Protection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerBitString = __cordl_object
            .invoke("get_Protection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Header(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader = __cordl_object
            .invoke("get_Header", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Body(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiBody,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiBody = __cordl_object
            .invoke("get_Body", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Sequence0(
        &mut self,
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PkiHeader_PkiBody_DerBitString_Il2CppArray1(
        &mut self,
        header: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        body: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiBody,
        protection: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
        extraCerts: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (header, body, protection, extraCerts))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PkiHeader_PkiBody_DerBitString2(
        &mut self,
        header: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        body: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiBody,
        protection: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (header, body, protection))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PkiHeader_PkiBody3(
        &mut self,
        header: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        body: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiBody,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (header, body))?;
        Ok(__cordl_ret)
    }
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_PkiHeader_PkiBody_DerBitString_Il2CppArray1(
        header: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        body: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiBody,
        protection: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
        extraCerts: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (header, body, protection, extraCerts))?;
        Ok(__cordl_object)
    }
    pub fn New_PkiHeader_PkiBody_DerBitString2(
        header: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        body: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiBody,
        protection: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (header, body, protection))?;
        Ok(__cordl_object)
    }
    pub fn New_PkiHeader_PkiBody3(
        header: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        body: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiBody,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (header, body))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
