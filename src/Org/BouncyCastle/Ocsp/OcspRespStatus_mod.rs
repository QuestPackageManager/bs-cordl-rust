#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspRespStatus")]
#[repr(C)]
#[derive(Debug)]
pub struct OcspRespStatus {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspRespStatus")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Ocsp::OcspRespStatus =>
    "Org.BouncyCastle.Ocsp"."OcspRespStatus"
);
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspRespStatus")]
impl std::ops::Deref for crate::Org::BouncyCastle::Ocsp::OcspRespStatus {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspRespStatus")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Ocsp::OcspRespStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspRespStatus")]
impl crate::Org::BouncyCastle::Ocsp::OcspRespStatus {
    pub const InternalError: i32 = 2i32;
    pub const MalformedRequest: i32 = 1i32;
    pub const SigRequired: i32 = 5i32;
    pub const Successful: i32 = 0i32;
    pub const TryLater: i32 = 3i32;
    pub const Unauthorized: i32 = 6i32;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+OcspRespStatus")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Ocsp::OcspRespStatus {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
