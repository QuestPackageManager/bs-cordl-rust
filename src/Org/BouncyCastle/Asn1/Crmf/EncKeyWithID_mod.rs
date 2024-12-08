#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncKeyWithID")]
#[repr(C)]
#[derive(Debug)]
pub struct EncKeyWithID {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub privKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
    pub identifier: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncKeyWithID")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Crmf::EncKeyWithID =>
    "Org.BouncyCastle.Asn1.Crmf"."EncKeyWithID"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncKeyWithID")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Crmf::EncKeyWithID {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncKeyWithID")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Crmf::EncKeyWithID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncKeyWithID")]
impl crate::Org::BouncyCastle::Asn1::Crmf::EncKeyWithID {
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
    pub fn _ctor_PrivateKeyInfo1(
        &mut self,
        privKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privKeyInfo))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PrivateKeyInfo_DerUtf8String2(
        &mut self,
        privKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
        str: *mut crate::Org::BouncyCastle::Asn1::DerUtf8String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privKeyInfo, str))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PrivateKeyInfo_GeneralName3(
        &mut self,
        privKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
        generalName: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privKeyInfo, generalName))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsIdentifierUtf8String(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsIdentifierUtf8String", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasIdentifier(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasIdentifier", ())?;
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
    pub fn get_Identifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable = __cordl_object
            .invoke("get_Identifier", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PrivateKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo = __cordl_object
            .invoke("get_PrivateKey", ())?;
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
    pub fn New_PrivateKeyInfo1(
        privKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privKeyInfo))?;
        Ok(__cordl_object)
    }
    pub fn New_PrivateKeyInfo_DerUtf8String2(
        privKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
        str: *mut crate::Org::BouncyCastle::Asn1::DerUtf8String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privKeyInfo, str))?;
        Ok(__cordl_object)
    }
    pub fn New_PrivateKeyInfo_GeneralName3(
        privKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
        generalName: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privKeyInfo, generalName))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncKeyWithID")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Crmf::EncKeyWithID {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
