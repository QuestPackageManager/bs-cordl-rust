#[cfg(feature = "System+Text+UTF8Encoding+UTF8EncodingSealed")]
#[repr(C)]
#[derive(Debug)]
pub struct UTF8Encoding_UTF8EncodingSealed {
    __cordl_parent: crate::System::Text::UTF8Encoding,
}
#[cfg(feature = "System+Text+UTF8Encoding+UTF8EncodingSealed")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::UTF8Encoding_UTF8EncodingSealed
    => "System.Text"."UTF8Encoding/UTF8EncodingSealed"
);
#[cfg(feature = "System+Text+UTF8Encoding+UTF8EncodingSealed")]
impl std::ops::Deref for crate::GlobalNamespace::UTF8Encoding_UTF8EncodingSealed {
    type Target = crate::System::Text::UTF8Encoding;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+UTF8Encoding+UTF8EncodingSealed")]
impl std::ops::DerefMut for crate::GlobalNamespace::UTF8Encoding_UTF8EncodingSealed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+UTF8Encoding+UTF8EncodingSealed")]
impl crate::GlobalNamespace::UTF8Encoding_UTF8EncodingSealed {
    pub fn _ctor(
        &mut self,
        encoderShouldEmitUTF8Identifier: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encoderShouldEmitUTF8Identifier))?;
        Ok(__cordl_ret)
    }
    pub fn get_Preamble(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = __cordl_object
            .invoke("get_Preamble", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        encoderShouldEmitUTF8Identifier: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encoderShouldEmitUTF8Identifier))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Text+UTF8Encoding+UTF8EncodingSealed")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::UTF8Encoding_UTF8EncodingSealed {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
