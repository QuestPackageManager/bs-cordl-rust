#[cfg(feature = "Org+BouncyCastle+Utilities+IO+TeeOutputStream")]
#[repr(C)]
#[derive(Debug)]
pub struct TeeOutputStream {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::BaseOutputStream,
    pub output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    pub tee: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+TeeOutputStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Utilities::IO::TeeOutputStream =>
    "Org.BouncyCastle.Utilities.IO"."TeeOutputStream"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+TeeOutputStream")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::IO::TeeOutputStream {
    type Target = crate::Org::BouncyCastle::Utilities::IO::BaseOutputStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+TeeOutputStream")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::IO::TeeOutputStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+TeeOutputStream")]
impl crate::Org::BouncyCastle::Utilities::IO::TeeOutputStream {
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        tee: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (output, tee))?;
        Ok(__cordl_object.into())
    }
    pub fn Write(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (buffer, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteByte(
        &mut self,
        b: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteByte", (b))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        tee: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (output, tee))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+TeeOutputStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::IO::TeeOutputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
