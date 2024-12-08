#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct PemWriter {
    __cordl_parent: crate::System::Object,
    pub writer: *mut crate::System::IO::TextWriter,
    pub nlLength: i32,
    pub buf: *mut quest_hook::libil2cpp::Il2CppArray<char>,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemWriter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Utilities::IO::Pem::PemWriter
    => "Org.BouncyCastle.Utilities.IO.Pem"."PemWriter"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemWriter")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::IO::Pem::PemWriter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemWriter")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::IO::Pem::PemWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemWriter")]
impl crate::Org::BouncyCastle::Utilities::IO::Pem::PemWriter {
    pub const LineLength: i32 = 64i32;
    pub fn GetOutputSize(
        &mut self,
        obj: *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetOutputSize", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn WriteEncoded(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEncoded", (bytes))?;
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
    pub fn WriteObject(
        &mut self,
        objGen: *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObjectGenerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteObject", (objGen))?;
        Ok(__cordl_ret)
    }
    pub fn WritePreEncapsulationBoundary(
        &mut self,
        _cordl_type: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WritePreEncapsulationBoundary", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn WritePostEncapsulationBoundary(
        &mut self,
        _cordl_type: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WritePostEncapsulationBoundary", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn get_Writer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::TextWriter> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::TextWriter = __cordl_object
            .invoke("get_Writer", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        writer: *mut crate::System::IO::TextWriter,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (writer))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemWriter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::IO::Pem::PemWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
