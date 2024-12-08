#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpLiteralDataGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpLiteralDataGenerator {
    __cordl_parent: crate::System::Object,
    pub pkOut: *mut crate::Org::BouncyCastle::Bcpg::BcpgOutputStream,
    pub oldFormat: bool,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpLiteralDataGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::OpenPgp::PgpLiteralDataGenerator =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."PgpLiteralDataGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpLiteralDataGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpLiteralDataGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpLiteralDataGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpLiteralDataGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpLiteralDataGenerator")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpLiteralDataGenerator {
    pub const Binary: char = 'b';
    pub const Console: &'static str = "_CONSOLE";
    pub const Text: char = 't';
    pub const Utf8: char = 'u';
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool1(
        oldFormat: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oldFormat))?;
        Ok(__cordl_object)
    }
    pub fn Open_FileInfo2(
        &mut self,
        outStr: *mut crate::System::IO::Stream,
        format: char,
        file: *mut crate::System::IO::FileInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("Open", (outStr, format, file))?;
        Ok(__cordl_ret)
    }
    pub fn Open_String_DateTime_Il2CppArray1(
        &mut self,
        outStr: *mut crate::System::IO::Stream,
        format: char,
        name: *mut crate::System::String,
        modificationTime: crate::System::DateTime,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("Open", (outStr, format, name, modificationTime, buffer))?;
        Ok(__cordl_ret)
    }
    pub fn Open_String_i64_DateTime0(
        &mut self,
        outStr: *mut crate::System::IO::Stream,
        format: char,
        name: *mut crate::System::String,
        length: i64,
        modificationTime: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("Open", (outStr, format, name, length, modificationTime))?;
        Ok(__cordl_ret)
    }
    pub fn WriteHeader(
        &mut self,
        outStr: *mut crate::Org::BouncyCastle::Bcpg::BcpgOutputStream,
        format: char,
        encName: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        modificationTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteHeader", (outStr, format, encName, modificationTime))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        oldFormat: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oldFormat))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpLiteralDataGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpLiteralDataGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}