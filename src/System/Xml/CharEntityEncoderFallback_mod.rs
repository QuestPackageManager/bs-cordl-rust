#[cfg(feature = "System+Xml+CharEntityEncoderFallback")]
#[repr(C)]
#[derive(Debug)]
pub struct CharEntityEncoderFallback {
    __cordl_parent: crate::System::Text::EncoderFallback,
    pub fallbackBuffer: *mut crate::System::Xml::CharEntityEncoderFallbackBuffer,
    pub textContentMarks: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub endMarkPos: i32,
    pub curMarkPos: i32,
    pub startOffset: i32,
}
#[cfg(feature = "System+Xml+CharEntityEncoderFallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::CharEntityEncoderFallback =>
    "System.Xml"."CharEntityEncoderFallback"
);
#[cfg(feature = "System+Xml+CharEntityEncoderFallback")]
impl std::ops::Deref for crate::System::Xml::CharEntityEncoderFallback {
    type Target = crate::System::Text::EncoderFallback;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+CharEntityEncoderFallback")]
impl std::ops::DerefMut for crate::System::Xml::CharEntityEncoderFallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+CharEntityEncoderFallback")]
impl crate::System::Xml::CharEntityEncoderFallback {
    pub fn CanReplaceAt(&mut self, index: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanReplaceAt", (index))?;
        Ok(__cordl_ret)
    }
    pub fn CreateFallbackBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::EncoderFallbackBuffer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::EncoderFallbackBuffer = __cordl_object
            .invoke("CreateFallbackBuffer", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Reset(
        &mut self,
        textContentMarks: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        endMarkPos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", (textContentMarks, endMarkPos))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MaxCharCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MaxCharCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_StartOffset(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_StartOffset", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+CharEntityEncoderFallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::CharEntityEncoderFallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
