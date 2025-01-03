#[cfg(feature = "Org+BouncyCastle+Asn1+LimitedInputStream")]
#[repr(C)]
#[derive(Debug)]
pub struct LimitedInputStream {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::BaseInputStream,
    pub _in: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    pub _limit: i32,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+LimitedInputStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::LimitedInputStream =>
    "Org.BouncyCastle.Asn1"."LimitedInputStream"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+LimitedInputStream")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::LimitedInputStream {
    type Target = crate::Org::BouncyCastle::Utilities::IO::BaseInputStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+LimitedInputStream")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::LimitedInputStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+LimitedInputStream")]
impl crate::Org::BouncyCastle::Asn1::LimitedInputStream {
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
    pub fn SetParentEofDetect(
        &mut self,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetParentEofDetect", (on))?;
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
    pub fn get_Limit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Limit", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+LimitedInputStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::LimitedInputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
