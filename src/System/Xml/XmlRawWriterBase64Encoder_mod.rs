#[cfg(feature = "System+Xml+XmlRawWriterBase64Encoder")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlRawWriterBase64Encoder {
    __cordl_parent: crate::System::Xml::Base64Encoder,
    pub rawWriter: *mut crate::System::Xml::XmlRawWriter,
}
#[cfg(feature = "System+Xml+XmlRawWriterBase64Encoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlRawWriterBase64Encoder =>
    "System.Xml"."XmlRawWriterBase64Encoder"
);
#[cfg(feature = "System+Xml+XmlRawWriterBase64Encoder")]
impl std::ops::Deref for crate::System::Xml::XmlRawWriterBase64Encoder {
    type Target = crate::System::Xml::Base64Encoder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlRawWriterBase64Encoder")]
impl std::ops::DerefMut for crate::System::Xml::XmlRawWriterBase64Encoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlRawWriterBase64Encoder")]
impl crate::System::Xml::XmlRawWriterBase64Encoder {
    pub fn New(
        rawWriter: *mut crate::System::Xml::XmlRawWriter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rawWriter))?;
        Ok(__cordl_object)
    }
    pub fn WriteChars(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteChars", (chars, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        rawWriter: *mut crate::System::Xml::XmlRawWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rawWriter))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlRawWriterBase64Encoder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlRawWriterBase64Encoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
