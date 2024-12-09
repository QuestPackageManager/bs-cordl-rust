#[cfg(feature = "Org+BouncyCastle+Pkcs+PkcsIOException")]
#[repr(C)]
#[derive(Debug)]
pub struct PkcsIOException {
    __cordl_parent: crate::System::IO::IOException,
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+PkcsIOException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Pkcs::PkcsIOException =>
    "Org.BouncyCastle.Pkcs"."PkcsIOException"
);
#[cfg(feature = "Org+BouncyCastle+Pkcs+PkcsIOException")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkcs::PkcsIOException {
    type Target = crate::System::IO::IOException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+PkcsIOException")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkcs::PkcsIOException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+PkcsIOException")]
impl crate::Org::BouncyCastle::Pkcs::PkcsIOException {
    pub fn New_Exception1(
        message: *mut quest_hook::libil2cpp::Il2CppString,
        underlying: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, underlying))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppString0(
        message: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Exception1(
        &mut self,
        message: *mut quest_hook::libil2cpp::Il2CppString,
        underlying: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, underlying))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        message: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+PkcsIOException")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkcs::PkcsIOException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
