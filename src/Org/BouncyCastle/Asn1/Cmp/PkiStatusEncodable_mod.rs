#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiStatusEncodable")]
#[repr(C)]
#[derive(Debug)]
pub struct PkiStatusEncodable {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub status: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiStatusEncodable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::PkiStatusEncodable
    => "Org.BouncyCastle.Asn1.Cmp"."PkiStatusEncodable"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiStatusEncodable")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusEncodable {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiStatusEncodable")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusEncodable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiStatusEncodable")]
impl crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusEncodable {
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_Value", ())?;
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
    pub fn _ctor_PkiStatus0(
        &mut self,
        status: crate::Org::BouncyCastle::Asn1::Cmp::PkiStatus,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (status))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerInteger1(
        &mut self,
        status: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (status))?;
        Ok(__cordl_ret)
    }
    pub fn New_PkiStatus0(
        status: crate::Org::BouncyCastle::Asn1::Cmp::PkiStatus,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (status))?;
        Ok(__cordl_object)
    }
    pub fn New_DerInteger1(
        status: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (status))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiStatusEncodable")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusEncodable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
