#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncryptedKey")]
#[repr(C)]
#[derive(Debug)]
pub struct EncryptedKey {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub envelopedData: *mut crate::Org::BouncyCastle::Asn1::Cms::EnvelopedData,
    pub encryptedValue: *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncryptedKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Crmf::EncryptedKey =>
    "Org.BouncyCastle.Asn1.Crmf"."EncryptedKey"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncryptedKey")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Crmf::EncryptedKey {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncryptedKey")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Crmf::EncryptedKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncryptedKey")]
impl crate::Org::BouncyCastle::Asn1::Crmf::EncryptedKey {
    pub fn New_EncryptedValue1(
        encryptedValue: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encryptedValue))?;
        Ok(__cordl_object.into())
    }
    pub fn New_EnvelopedData0(
        envelopedData: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::EnvelopedData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (envelopedData))?;
        Ok(__cordl_object.into())
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = __cordl_object.invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_EncryptedValue1(
        &mut self,
        encryptedValue: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encryptedValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_EnvelopedData0(
        &mut self,
        envelopedData: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::EnvelopedData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (envelopedData))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsEncryptedValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEncryptedValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Encodable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Encodable,
        > = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncryptedKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Crmf::EncryptedKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncryptedKey")]
impl AsRef<crate::Org::BouncyCastle::Asn1::IAsn1Choice>
for crate::Org::BouncyCastle::Asn1::Crmf::EncryptedKey {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Asn1::IAsn1Choice {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncryptedKey")]
impl AsMut<crate::Org::BouncyCastle::Asn1::IAsn1Choice>
for crate::Org::BouncyCastle::Asn1::Crmf::EncryptedKey {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Asn1::IAsn1Choice {
        unsafe { std::mem::transmute(self) }
    }
}
