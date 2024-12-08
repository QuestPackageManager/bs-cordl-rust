#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspResponseStatus")]
#[repr(C)]
#[derive(Debug)]
pub struct OcspResponseStatus {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::DerEnumerated,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspResponseStatus")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Ocsp::OcspResponseStatus => "Org.BouncyCastle.Asn1.Ocsp"
    ."OcspResponseStatus"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspResponseStatus")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Ocsp::OcspResponseStatus {
    type Target = crate::Org::BouncyCastle::Asn1::DerEnumerated;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspResponseStatus")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Ocsp::OcspResponseStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspResponseStatus")]
impl crate::Org::BouncyCastle::Asn1::Ocsp::OcspResponseStatus {
    pub const InternalError: i32 = 2i32;
    pub const MalformedRequest: i32 = 1i32;
    pub const SignatureRequired: i32 = 5i32;
    pub const Successful: i32 = 0i32;
    pub const TryLater: i32 = 3i32;
    pub const Unauthorized: i32 = 6i32;
    pub fn _ctor_i32_0(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerEnumerated1(
        &mut self,
        value: *mut crate::Org::BouncyCastle::Asn1::DerEnumerated,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New_i32_0(value: i32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value))?;
        Ok(__cordl_object)
    }
    pub fn New_DerEnumerated1(
        value: *mut crate::Org::BouncyCastle::Asn1::DerEnumerated,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspResponseStatus")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Ocsp::OcspResponseStatus {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
