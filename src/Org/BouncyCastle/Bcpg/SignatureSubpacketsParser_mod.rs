#[cfg(feature = "Org+BouncyCastle+Bcpg+SignatureSubpacketsParser")]
#[repr(C)]
#[derive(Debug)]
pub struct SignatureSubpacketsParser {
    __cordl_parent: crate::System::Object,
    pub input: *mut crate::System::IO::Stream,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SignatureSubpacketsParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::SignatureSubpacketsParser => "Org.BouncyCastle.Bcpg"
    ."SignatureSubpacketsParser"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+SignatureSubpacketsParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::SignatureSubpacketsParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SignatureSubpacketsParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::SignatureSubpacketsParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SignatureSubpacketsParser")]
impl crate::Org::BouncyCastle::Bcpg::SignatureSubpacketsParser {
    pub fn CheckData(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        expected: i32,
        bytesRead: i32,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("CheckData", (data, expected, bytesRead, name))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        input: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input))?;
        Ok(__cordl_object)
    }
    pub fn ReadPacket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Bcpg::SignatureSubpacket = __cordl_object
            .invoke("ReadPacket", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        input: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SignatureSubpacketsParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::SignatureSubpacketsParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
