#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct PemWriter {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::Pem::PemWriter,
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemWriter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::OpenSsl::PemWriter =>
    "Org.BouncyCastle.OpenSsl"."PemWriter"
);
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemWriter")]
impl std::ops::Deref for crate::Org::BouncyCastle::OpenSsl::PemWriter {
    type Target = crate::Org::BouncyCastle::Utilities::IO::Pem::PemWriter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemWriter")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::OpenSsl::PemWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemWriter")]
impl crate::Org::BouncyCastle::OpenSsl::PemWriter {
    pub fn New(
        writer: *mut crate::System::IO::TextWriter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (writer))?;
        Ok(__cordl_object)
    }
    pub fn WriteObject_Il2CppObject0(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteObject", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn WriteObject_Il2CppString_Il2CppArray_SecureRandom1(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
        algorithm: *mut quest_hook::libil2cpp::Il2CppString,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteObject", (obj, algorithm, password, random))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        writer: *mut crate::System::IO::TextWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (writer))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+PemWriter")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::OpenSsl::PemWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
