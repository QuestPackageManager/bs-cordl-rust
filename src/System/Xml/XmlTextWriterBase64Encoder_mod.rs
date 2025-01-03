#[cfg(feature = "System+Xml+XmlTextWriterBase64Encoder")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTextWriterBase64Encoder {
    __cordl_parent: crate::System::Xml::Base64Encoder,
    pub xmlTextEncoder: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlTextEncoder>,
}
#[cfg(feature = "System+Xml+XmlTextWriterBase64Encoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextWriterBase64Encoder =>
    "System.Xml"."XmlTextWriterBase64Encoder"
);
#[cfg(feature = "System+Xml+XmlTextWriterBase64Encoder")]
impl std::ops::Deref for crate::System::Xml::XmlTextWriterBase64Encoder {
    type Target = crate::System::Xml::Base64Encoder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextWriterBase64Encoder")]
impl std::ops::DerefMut for crate::System::Xml::XmlTextWriterBase64Encoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextWriterBase64Encoder")]
impl crate::System::Xml::XmlTextWriterBase64Encoder {
    pub fn New(
        xmlTextEncoder: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlTextEncoder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlTextEncoder))?;
        Ok(__cordl_object.into())
    }
    pub fn WriteChars(
        &mut self,
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteChars", (chars, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        xmlTextEncoder: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlTextEncoder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlTextEncoder))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlTextWriterBase64Encoder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlTextWriterBase64Encoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
