#[cfg(feature = "Org+BouncyCastle+Cms+CmsCompressedDataParser")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsCompressedDataParser {
    __cordl_parent: crate::Org::BouncyCastle::Cms::CmsContentInfoParser,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsCompressedDataParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::CmsCompressedDataParser
    => "Org.BouncyCastle.Cms"."CmsCompressedDataParser"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsCompressedDataParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsCompressedDataParser {
    type Target = crate::Org::BouncyCastle::Cms::CmsContentInfoParser;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsCompressedDataParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsCompressedDataParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsCompressedDataParser")]
impl crate::Org::BouncyCastle::Cms::CmsCompressedDataParser {
    pub fn GetContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsTypedStream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsTypedStream,
        > = __cordl_object.invoke("GetContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppArray0(
        compressedData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (compressedData))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Stream1(
        compressedData: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (compressedData))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        compressedData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (compressedData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Stream1(
        &mut self,
        compressedData: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (compressedData))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsCompressedDataParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsCompressedDataParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
