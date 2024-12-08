#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+Ecc+MQVuserKeyingMaterial")]
#[repr(C)]
#[derive(Debug)]
pub struct MQVuserKeyingMaterial {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub ephemeralPublicKey: *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorPublicKey,
    pub addedukm: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+Ecc+MQVuserKeyingMaterial")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Cms::Ecc::MQVuserKeyingMaterial =>
    "Org.BouncyCastle.Asn1.Cms.Ecc"."MQVuserKeyingMaterial"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+Ecc+MQVuserKeyingMaterial")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Cms::Ecc::MQVuserKeyingMaterial {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+Ecc+MQVuserKeyingMaterial")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Cms::Ecc::MQVuserKeyingMaterial {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+Ecc+MQVuserKeyingMaterial")]
impl crate::Org::BouncyCastle::Asn1::Cms::Ecc::MQVuserKeyingMaterial {
    pub fn get_AddedUkm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("get_AddedUkm", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_OriginatorPublicKey_Asn1OctetString0(
        &mut self,
        ephemeralPublicKey: *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorPublicKey,
        addedukm: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ephemeralPublicKey, addedukm))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Sequence1(
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
    pub fn get_EphemeralPublicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorPublicKey,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorPublicKey = __cordl_object
            .invoke("get_EphemeralPublicKey", ())?;
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
    pub fn New_OriginatorPublicKey_Asn1OctetString0(
        ephemeralPublicKey: *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorPublicKey,
        addedukm: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ephemeralPublicKey, addedukm))?;
        Ok(__cordl_object)
    }
    pub fn New_Asn1Sequence1(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+Ecc+MQVuserKeyingMaterial")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cms::Ecc::MQVuserKeyingMaterial {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
