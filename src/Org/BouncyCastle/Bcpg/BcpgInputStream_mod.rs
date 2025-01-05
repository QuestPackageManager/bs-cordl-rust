#[cfg(feature = "Org+BouncyCastle+Bcpg+BcpgInputStream")]
#[repr(C)]
#[derive(Debug)]
pub struct BcpgInputStream {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::BaseInputStream,
    pub m_in: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    pub next: bool,
    pub nextB: i32,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+BcpgInputStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::BcpgInputStream =>
    "Org.BouncyCastle.Bcpg"."BcpgInputStream"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+BcpgInputStream")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::BcpgInputStream {
    type Target = crate::Org::BouncyCastle::Utilities::IO::BaseInputStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+BcpgInputStream")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::BcpgInputStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+BcpgInputStream")]
impl crate::Org::BouncyCastle::Bcpg::BcpgInputStream {
    #[cfg(feature = "Org+BouncyCastle+Bcpg+BcpgInputStream+PartialInputStream")]
    pub type PartialInputStream = crate::Org::BouncyCastle::Bcpg::BcpgInputStream_PartialInputStream;
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
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (inputStream))?;
        Ok(__cordl_object.into())
    }
    pub fn NextPacketTag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Org::BouncyCastle::Bcpg::PacketTag> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Org::BouncyCastle::Bcpg::PacketTag = __cordl_object
            .invoke("NextPacketTag", ())?;
        Ok(__cordl_ret.into())
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
    pub fn ReadAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("ReadAll", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadByte(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadByte", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadFully_Il2CppArray1(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadFully", (buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadFully_i32_i32_0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadFully", (buffer, off, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadPacket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::Packet>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::Packet,
        > = __cordl_object.invoke("ReadPacket", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Wrap(
        inStr: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::BcpgInputStream>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Wrap", (inStr))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (inputStream))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+BcpgInputStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::BcpgInputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+BcpgInputStream+PartialInputStream")]
#[repr(C)]
#[derive(Debug)]
pub struct BcpgInputStream_PartialInputStream {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::BaseInputStream,
    pub m_in: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::BcpgInputStream>,
    pub partial: bool,
    pub dataLength: i32,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+BcpgInputStream+PartialInputStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::BcpgInputStream_PartialInputStream =>
    "Org.BouncyCastle.Bcpg"."BcpgInputStream/PartialInputStream"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+BcpgInputStream+PartialInputStream")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Bcpg::BcpgInputStream_PartialInputStream {
    type Target = crate::Org::BouncyCastle::Utilities::IO::BaseInputStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+BcpgInputStream+PartialInputStream")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::BcpgInputStream_PartialInputStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+BcpgInputStream+PartialInputStream")]
impl crate::Org::BouncyCastle::Bcpg::BcpgInputStream_PartialInputStream {
    pub fn New(
        bcpgIn: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
        >,
        partial: bool,
        dataLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bcpgIn, partial, dataLength))?;
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
    pub fn ReadPartialDataLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadPartialDataLength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bcpgIn: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
        >,
        partial: bool,
        dataLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bcpgIn, partial, dataLength))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+BcpgInputStream+PartialInputStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::BcpgInputStream_PartialInputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
