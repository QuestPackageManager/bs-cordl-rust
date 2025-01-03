#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+PopoSigningKeyInput")]
#[repr(C)]
#[derive(Debug)]
pub struct PopoSigningKeyInput {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub sender: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    >,
    pub publicKeyMac: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Crmf::PKMacValue,
    >,
    pub publicKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+PopoSigningKeyInput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Crmf::PopoSigningKeyInput => "Org.BouncyCastle.Asn1.Crmf"
    ."PopoSigningKeyInput"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+PopoSigningKeyInput")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Crmf::PopoSigningKeyInput {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+PopoSigningKeyInput")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Crmf::PopoSigningKeyInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+PopoSigningKeyInput")]
impl crate::Org::BouncyCastle::Asn1::Crmf::PopoSigningKeyInput {
    pub fn GetInstance(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Crmf::PopoSigningKeyInput,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Crmf::PopoSigningKeyInput,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetInstance", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Asn1Sequence0(
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object.into())
    }
    pub fn New_GeneralName_SubjectPublicKeyInfo1(
        sender: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        >,
        spki: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sender, spki))?;
        Ok(__cordl_object.into())
    }
    pub fn New_PKMacValue_SubjectPublicKeyInfo2(
        pkmac: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Crmf::PKMacValue,
        >,
        spki: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pkmac, spki))?;
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
    pub fn _ctor_Asn1Sequence0(
        &mut self,
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GeneralName_SubjectPublicKeyInfo1(
        &mut self,
        sender: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        >,
        spki: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sender, spki))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PKMacValue_SubjectPublicKeyInfo2(
        &mut self,
        pkmac: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Crmf::PKMacValue,
        >,
        spki: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pkmac, spki))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PublicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
        > = __cordl_object.invoke("get_PublicKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PublicKeyMac(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Crmf::PKMacValue>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Crmf::PKMacValue,
        > = __cordl_object.invoke("get_PublicKeyMac", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Sender(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::GeneralName>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        > = __cordl_object.invoke("get_Sender", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+PopoSigningKeyInput")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Crmf::PopoSigningKeyInput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
