#[cfg(feature = "Org+BouncyCastle+Asn1+ConstructedOctetStream")]
#[repr(C)]
#[derive(Debug)]
pub struct ConstructedOctetStream {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::IO::BaseInputStream,
    >,
    pub _parser: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1StreamParser,
    >,
    pub _first: bool,
    pub _currentStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+ConstructedOctetStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::ConstructedOctetStream
    => "Org.BouncyCastle.Asn1"."ConstructedOctetStream"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+ConstructedOctetStream")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::ConstructedOctetStream {
    type Target = quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::IO::BaseInputStream,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+ConstructedOctetStream")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::ConstructedOctetStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+ConstructedOctetStream")]
impl crate::Org::BouncyCastle::Asn1::ConstructedOctetStream {
    pub fn GetNextParser(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1OctetStringParser>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1OctetStringParser,
        > = __cordl_object.invoke("GetNextParser", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        parser: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1StreamParser,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parser))?;
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
    pub fn _ctor(
        &mut self,
        parser: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1StreamParser,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parser))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+ConstructedOctetStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::ConstructedOctetStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
