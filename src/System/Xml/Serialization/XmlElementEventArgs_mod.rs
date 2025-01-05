#[cfg(feature = "System+Xml+Serialization+XmlElementEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlElementEventArgs {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::EventArgs>,
    pub o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub elem: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    pub qnames: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
    type Target = quest_hook::libil2cpp::Gc<crate::System::EventArgs>;
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
    pub fn New(
        elem: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        lineNumber: i32,
        linePosition: i32,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        qnames: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (elem, lineNumber, linePosition, o, qnames))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        elem: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        lineNumber: i32,
        linePosition: i32,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        qnames: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (elem, lineNumber, linePosition, o, qnames))?;
        Ok(__cordl_ret.into())
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
