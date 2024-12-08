#[cfg(feature = "Org+BouncyCastle+Crmf+EncryptedValueBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct EncryptedValueBuilder {
    __cordl_parent: crate::System::Object,
    pub wrapper: *mut crate::Org::BouncyCastle::Crypto::IKeyWrapper,
    pub encryptor: *mut crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey,
    pub padder: *mut crate::Org::BouncyCastle::Crmf::IEncryptedValuePadder,
}
#[cfg(feature = "Org+BouncyCastle+Crmf+EncryptedValueBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crmf::EncryptedValueBuilder
    => "Org.BouncyCastle.Crmf"."EncryptedValueBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Crmf+EncryptedValueBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crmf::EncryptedValueBuilder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+EncryptedValueBuilder")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crmf::EncryptedValueBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+EncryptedValueBuilder")]
impl crate::Org::BouncyCastle::Crmf::EncryptedValueBuilder {
    pub fn Build_Il2CppArray0(
        &mut self,
        revocationPassphrase: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue = __cordl_object
            .invoke("Build", (revocationPassphrase))?;
        Ok(__cordl_ret)
    }
    pub fn Build_X509Certificate1(
        &mut self,
        holder: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue = __cordl_object
            .invoke("Build", (holder))?;
        Ok(__cordl_ret)
    }
    pub fn Build_PrivateKeyInfo2(
        &mut self,
        privateKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue = __cordl_object
            .invoke("Build", (privateKeyInfo))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IKeyWrapper_ICipherBuilderWithKey0(
        &mut self,
        wrapper: *mut crate::Org::BouncyCastle::Crypto::IKeyWrapper,
        encryptor: *mut crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (wrapper, encryptor))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IEncryptedValuePadder1(
        &mut self,
        wrapper: *mut crate::Org::BouncyCastle::Crypto::IKeyWrapper,
        encryptor: *mut crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey,
        padder: *mut crate::Org::BouncyCastle::Crmf::IEncryptedValuePadder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (wrapper, encryptor, padder))?;
        Ok(__cordl_ret)
    }
    pub fn EncryptData(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue = __cordl_object
            .invoke("EncryptData", (data))?;
        Ok(__cordl_ret)
    }
    pub fn PadData(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("PadData", (data))?;
        Ok(__cordl_ret)
    }
    pub fn New_IKeyWrapper_ICipherBuilderWithKey0(
        wrapper: *mut crate::Org::BouncyCastle::Crypto::IKeyWrapper,
        encryptor: *mut crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (wrapper, encryptor))?;
        Ok(__cordl_object)
    }
    pub fn New_IEncryptedValuePadder1(
        wrapper: *mut crate::Org::BouncyCastle::Crypto::IKeyWrapper,
        encryptor: *mut crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey,
        padder: *mut crate::Org::BouncyCastle::Crmf::IEncryptedValuePadder,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (wrapper, encryptor, padder))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+EncryptedValueBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crmf::EncryptedValueBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
