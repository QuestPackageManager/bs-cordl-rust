#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiBody")]
#[repr(C)]
#[derive(Debug)]
pub struct PkiBody {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub tagNo: i32,
    pub body: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiBody")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::PkiBody =>
    "Org.BouncyCastle.Asn1.Cmp"."PkiBody"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiBody")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::PkiBody {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiBody")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::PkiBody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiBody")]
impl crate::Org::BouncyCastle::Asn1::Cmp::PkiBody {
    pub const TYPE_CA_KEY_UPDATE_ANN: i32 = 15i32;
    pub const TYPE_CERT_ANN: i32 = 16i32;
    pub const TYPE_CERT_CONFIRM: i32 = 24i32;
    pub const TYPE_CERT_REP: i32 = 3i32;
    pub const TYPE_CERT_REQ: i32 = 2i32;
    pub const TYPE_CONFIRM: i32 = 19i32;
    pub const TYPE_CRL_ANN: i32 = 18i32;
    pub const TYPE_CROSS_CERT_REP: i32 = 14i32;
    pub const TYPE_CROSS_CERT_REQ: i32 = 13i32;
    pub const TYPE_ERROR: i32 = 23i32;
    pub const TYPE_GEN_MSG: i32 = 21i32;
    pub const TYPE_GEN_REP: i32 = 22i32;
    pub const TYPE_INIT_REP: i32 = 1i32;
    pub const TYPE_INIT_REQ: i32 = 0i32;
    pub const TYPE_KEY_RECOVERY_REP: i32 = 10i32;
    pub const TYPE_KEY_RECOVERY_REQ: i32 = 9i32;
    pub const TYPE_KEY_UPDATE_REP: i32 = 8i32;
    pub const TYPE_KEY_UPDATE_REQ: i32 = 7i32;
    pub const TYPE_NESTED: i32 = 20i32;
    pub const TYPE_P10_CERT_REQ: i32 = 4i32;
    pub const TYPE_POLL_REP: i32 = 26i32;
    pub const TYPE_POLL_REQ: i32 = 25i32;
    pub const TYPE_POPO_CHALL: i32 = 5i32;
    pub const TYPE_POPO_REP: i32 = 6i32;
    pub const TYPE_REVOCATION_ANN: i32 = 17i32;
    pub const TYPE_REVOCATION_REP: i32 = 12i32;
    pub const TYPE_REVOCATION_REQ: i32 = 11i32;
    pub fn New_Asn1TaggedObject0(
        tagged: *mut crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tagged))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_Asn1Encodable1(
        _cordl_type: i32,
        content: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, content))?;
        Ok(__cordl_object)
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
    pub fn _ctor_i32_Asn1Encodable1(
        &mut self,
        _cordl_type: i32,
        content: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, content))?;
        Ok(__cordl_ret)
    }
    pub fn get_Content(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable = __cordl_object
            .invoke("get_Content", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Type", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiBody")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Asn1::Cmp::PkiBody {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
