#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+ProofOfPossession")]
#[repr(C)]
#[derive(Debug)]
pub struct ProofOfPossession {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub tagNo: i32,
    pub obj: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+ProofOfPossession")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Crmf::ProofOfPossession
    => "Org.BouncyCastle.Asn1.Crmf"."ProofOfPossession"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+ProofOfPossession")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Crmf::ProofOfPossession {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+ProofOfPossession")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Crmf::ProofOfPossession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+ProofOfPossession")]
impl crate::Org::BouncyCastle::Asn1::Crmf::ProofOfPossession {
    pub const TYPE_KEY_AGREEMENT: i32 = 3i32;
    pub const TYPE_KEY_ENCIPHERMENT: i32 = 2i32;
    pub const TYPE_RA_VERIFIED: i32 = 0i32;
    pub const TYPE_SIGNING_KEY: i32 = 1i32;
    pub fn get_Type(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Type", ())?;
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
    pub fn _ctor_Asn1TaggedObject0(
        &mut self,
        tagged: *mut crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tagged))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PopoSigningKey2(
        &mut self,
        Poposk: *mut crate::Org::BouncyCastle::Asn1::Crmf::PopoSigningKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (Poposk))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_PopoPrivKey3(
        &mut self,
        _cordl_type: i32,
        privkey: *mut crate::Org::BouncyCastle::Asn1::Crmf::PopoPrivKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, privkey))?;
        Ok(__cordl_ret)
    }
    pub fn get_Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable = __cordl_object
            .invoke("get_Object", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Asn1TaggedObject0(
        tagged: *mut crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tagged))?;
        Ok(__cordl_object)
    }
    pub fn New_1() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_PopoSigningKey2(
        Poposk: *mut crate::Org::BouncyCastle::Asn1::Crmf::PopoSigningKey,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (Poposk))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_PopoPrivKey3(
        _cordl_type: i32,
        privkey: *mut crate::Org::BouncyCastle::Asn1::Crmf::PopoPrivKey,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, privkey))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+ProofOfPossession")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Crmf::ProofOfPossession {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
