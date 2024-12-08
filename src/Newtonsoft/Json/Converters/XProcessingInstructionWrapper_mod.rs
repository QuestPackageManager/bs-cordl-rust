#[cfg(feature = "Newtonsoft+Json+Converters+XProcessingInstructionWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct XProcessingInstructionWrapper {
    __cordl_parent: crate::Newtonsoft::Json::Converters::XObjectWrapper,
}
#[cfg(feature = "Newtonsoft+Json+Converters+XProcessingInstructionWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Converters::XProcessingInstructionWrapper =>
    "Newtonsoft.Json.Converters"."XProcessingInstructionWrapper"
);
#[cfg(feature = "Newtonsoft+Json+Converters+XProcessingInstructionWrapper")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Converters::XProcessingInstructionWrapper {
    type Target = crate::Newtonsoft::Json::Converters::XObjectWrapper;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XProcessingInstructionWrapper")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Converters::XProcessingInstructionWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XProcessingInstructionWrapper")]
impl crate::Newtonsoft::Json::Converters::XProcessingInstructionWrapper {
    pub fn get_ProcessingInstruction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Linq::XProcessingInstruction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Linq::XProcessingInstruction = __cordl_object
            .invoke("get_ProcessingInstruction", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Value(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Value", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        processingInstruction: *mut crate::System::Xml::Linq::XProcessingInstruction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (processingInstruction))?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_LocalName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Value", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        processingInstruction: *mut crate::System::Xml::Linq::XProcessingInstruction,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (processingInstruction))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XProcessingInstructionWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::XProcessingInstructionWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
