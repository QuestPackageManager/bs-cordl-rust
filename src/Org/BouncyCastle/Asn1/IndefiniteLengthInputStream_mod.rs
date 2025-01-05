#[cfg(feature = "Org+BouncyCastle+Asn1+IndefiniteLengthInputStream")]
#[repr(C)]
#[derive(Debug)]
pub struct IndefiniteLengthInputStream {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::LimitedInputStream,
    pub _lookAhead: i32,
    pub _eofOn00: bool,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IndefiniteLengthInputStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::IndefiniteLengthInputStream => "Org.BouncyCastle.Asn1"
    ."IndefiniteLengthInputStream"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+IndefiniteLengthInputStream")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::IndefiniteLengthInputStream {
    type Target = crate::Org::BouncyCastle::Asn1::LimitedInputStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IndefiniteLengthInputStream")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::IndefiniteLengthInputStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IndefiniteLengthInputStream")]
impl crate::Org::BouncyCastle::Asn1::IndefiniteLengthInputStream {
    pub fn CheckForEof(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckForEof", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        inStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        limit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (inStream, limit))?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Read", (buffer, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadByte(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadByte", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RequireByte(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("RequireByte", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetEofOn00(
        &mut self,
        eofOn00: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetEofOn00", (eofOn00))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        inStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        limit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (inStream, limit))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IndefiniteLengthInputStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::IndefiniteLengthInputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
