#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemReader")]
#[repr(C)]
#[derive(Debug)]
pub struct PemReader {
    __cordl_parent: crate::System::Object,
    pub reader: *mut crate::System::IO::TextReader,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Utilities::IO::Pem::PemReader
    => "Org.BouncyCastle.Utilities.IO.Pem"."PemReader"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemReader")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::IO::Pem::PemReader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemReader")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::IO::Pem::PemReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemReader")]
impl crate::Org::BouncyCastle::Utilities::IO::Pem::PemReader {
    pub const BeginString: &'static str = "-----BEGIN ";
    pub const EndString: &'static str = "-----END ";
    pub fn LoadObject(
        &mut self,
        _cordl_type: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject = __cordl_object
            .invoke("LoadObject", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        reader: *mut crate::System::IO::TextReader,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reader))?;
        Ok(__cordl_object)
    }
    pub fn ReadPemObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject = __cordl_object
            .invoke("ReadPemObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        reader: *mut crate::System::IO::TextReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn get_Reader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::TextReader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::TextReader = __cordl_object
            .invoke("get_Reader", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemReader")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::IO::Pem::PemReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
