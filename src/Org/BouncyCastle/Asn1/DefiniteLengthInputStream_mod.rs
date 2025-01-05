#[cfg(feature = "Org+BouncyCastle+Asn1+DefiniteLengthInputStream")]
#[repr(C)]
#[derive(Debug)]
pub struct DefiniteLengthInputStream {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::LimitedInputStream,
    pub _originalLength: i32,
    pub _remaining: i32,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DefiniteLengthInputStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::DefiniteLengthInputStream => "Org.BouncyCastle.Asn1"
    ."DefiniteLengthInputStream"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+DefiniteLengthInputStream")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::DefiniteLengthInputStream {
    type Target = crate::Org::BouncyCastle::Asn1::LimitedInputStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DefiniteLengthInputStream")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::DefiniteLengthInputStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DefiniteLengthInputStream")]
impl crate::Org::BouncyCastle::Asn1::DefiniteLengthInputStream {
    pub fn New(
        inStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        length: i32,
        limit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (inStream, length, limit))?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Read", (buf, off, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAllIntoByteArray(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadAllIntoByteArray", (buf))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadByte(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadByte", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("ToArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        inStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        length: i32,
        limit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (inStream, length, limit))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Remaining(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Remaining", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DefiniteLengthInputStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::DefiniteLengthInputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
