#[cfg(feature = "Org+BouncyCastle+X509+PemParser")]
#[repr(C)]
#[derive(Debug)]
pub struct PemParser {
    __cordl_parent: crate::System::Object,
    pub _header1: *mut crate::System::String,
    pub _header2: *mut crate::System::String,
    pub _footer1: *mut crate::System::String,
    pub _footer2: *mut crate::System::String,
}
#[cfg(feature = "Org+BouncyCastle+X509+PemParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::X509::PemParser =>
    "Org.BouncyCastle.X509"."PemParser"
);
#[cfg(feature = "Org+BouncyCastle+X509+PemParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::X509::PemParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+PemParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::X509::PemParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+PemParser")]
impl crate::Org::BouncyCastle::X509::PemParser {
    pub fn ReadLine(
        &mut self,
        inStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadLine", (inStream))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn ReadPemObject(
        &mut self,
        inStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence = __cordl_object
            .invoke("ReadPemObject", (inStream))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_type: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+PemParser")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::X509::PemParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
