#[cfg(feature = "System+Xml+Serialization+XmlAttributeEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlAttributeEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub o: *mut quest_hook::libil2cpp::Il2CppObject,
    pub attr: *mut crate::System::Xml::XmlAttribute,
    pub qnames: *mut quest_hook::libil2cpp::Il2CppString,
    pub lineNumber: i32,
    pub linePosition: i32,
}
#[cfg(feature = "System+Xml+Serialization+XmlAttributeEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlAttributeEventArgs => "System.Xml.Serialization"
    ."XmlAttributeEventArgs"
);
#[cfg(feature = "System+Xml+Serialization+XmlAttributeEventArgs")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlAttributeEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlAttributeEventArgs")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlAttributeEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlAttributeEventArgs")]
impl crate::System::Xml::Serialization::XmlAttributeEventArgs {
    pub fn New(
        attr: *mut crate::System::Xml::XmlAttribute,
        lineNumber: i32,
        linePosition: i32,
        o: *mut quest_hook::libil2cpp::Il2CppObject,
        qnames: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (attr, lineNumber, linePosition, o, qnames))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        attr: *mut crate::System::Xml::XmlAttribute,
        lineNumber: i32,
        linePosition: i32,
        o: *mut quest_hook::libil2cpp::Il2CppObject,
        qnames: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (attr, lineNumber, linePosition, o, qnames))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlAttributeEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlAttributeEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
