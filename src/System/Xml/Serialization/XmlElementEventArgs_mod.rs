#[cfg(feature = "System+Xml+Serialization+XmlElementEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlElementEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub o: *mut crate::System::Object,
    pub elem: *mut crate::System::Xml::XmlElement,
    pub qnames: *mut crate::System::String,
    pub lineNumber: i32,
    pub linePosition: i32,
}
#[cfg(feature = "System+Xml+Serialization+XmlElementEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::XmlElementEventArgs
    => "System.Xml.Serialization"."XmlElementEventArgs"
);
#[cfg(feature = "System+Xml+Serialization+XmlElementEventArgs")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlElementEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlElementEventArgs")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlElementEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlElementEventArgs")]
impl crate::System::Xml::Serialization::XmlElementEventArgs {
    pub fn _ctor(
        &mut self,
        elem: *mut crate::System::Xml::XmlElement,
        lineNumber: i32,
        linePosition: i32,
        o: *mut crate::System::Object,
        qnames: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (elem, lineNumber, linePosition, o, qnames))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        elem: *mut crate::System::Xml::XmlElement,
        lineNumber: i32,
        linePosition: i32,
        o: *mut crate::System::Object,
        qnames: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (elem, lineNumber, linePosition, o, qnames))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlElementEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlElementEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
