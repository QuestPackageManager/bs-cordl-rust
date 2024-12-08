#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+PopoPrivKey")]
#[repr(C)]
#[derive(Debug)]
pub struct PopoPrivKey {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub tagNo: i32,
    pub obj: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+PopoPrivKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Crmf::PopoPrivKey =>
    "Org.BouncyCastle.Asn1.Crmf"."PopoPrivKey"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+PopoPrivKey")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Crmf::PopoPrivKey {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+PopoPrivKey")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Crmf::PopoPrivKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+PopoPrivKey")]
impl crate::Org::BouncyCastle::Asn1::Crmf::PopoPrivKey {
    pub const agreeMAC: i32 = 3i32;
    pub const dhMAC: i32 = 2i32;
    pub const encryptedKey: i32 = 4i32;
    pub const subsequentMessage: i32 = 1i32;
    pub const thisMessage: i32 = 0i32;
    pub fn get_Type(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Type", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable = __cordl_object
            .invoke("get_Value", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1TaggedObject0(
        &mut self,
        obj: *mut crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SubsequentMessage1(
        &mut self,
        msg: *mut crate::Org::BouncyCastle::Asn1::Crmf::SubsequentMessage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (msg))?;
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
    pub fn New_Asn1TaggedObject0(
        obj: *mut crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (obj))?;
        Ok(__cordl_object)
    }
    pub fn New_SubsequentMessage1(
        msg: *mut crate::Org::BouncyCastle::Asn1::Crmf::SubsequentMessage,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (msg))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+PopoPrivKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Crmf::PopoPrivKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
