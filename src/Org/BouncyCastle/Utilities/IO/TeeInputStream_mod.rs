#[cfg(feature = "Org+BouncyCastle+Utilities+IO+TeeInputStream")]
#[repr(C)]
#[derive(Debug)]
pub struct TeeInputStream {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::BaseInputStream,
    pub input: *mut crate::System::IO::Stream,
    pub tee: *mut crate::System::IO::Stream,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+TeeInputStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Utilities::IO::TeeInputStream
    => "Org.BouncyCastle.Utilities.IO"."TeeInputStream"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+TeeInputStream")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::IO::TeeInputStream {
    type Target = crate::Org::BouncyCastle::Utilities::IO::BaseInputStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+TeeInputStream")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::IO::TeeInputStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+TeeInputStream")]
impl crate::Org::BouncyCastle::Utilities::IO::TeeInputStream {
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
    pub fn Read(
        &mut self,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Read", (buf, off, len))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        input: *mut crate::System::IO::Stream,
        tee: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input, tee))?;
        Ok(__cordl_ret)
    }
    pub fn ReadByte(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadByte", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        input: *mut crate::System::IO::Stream,
        tee: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input, tee))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+TeeInputStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::IO::TeeInputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
