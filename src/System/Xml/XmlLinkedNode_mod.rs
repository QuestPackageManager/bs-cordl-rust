#[cfg(feature = "System+Xml+XmlLinkedNode")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlLinkedNode {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    pub next: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlLinkedNode>,
}
#[cfg(feature = "System+Xml+XmlLinkedNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlLinkedNode => "System.Xml"
    ."XmlLinkedNode"
);
#[cfg(feature = "System+Xml+XmlLinkedNode")]
impl std::ops::Deref for crate::System::Xml::XmlLinkedNode {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlLinkedNode")]
impl std::ops::DerefMut for crate::System::Xml::XmlLinkedNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlLinkedNode")]
impl crate::System::Xml::XmlLinkedNode {
    pub fn New(
        doc: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (doc))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        doc: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (doc))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NextSibling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode> = __cordl_object
            .invoke("get_NextSibling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PreviousSibling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode> = __cordl_object
            .invoke("get_PreviousSibling", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlLinkedNode")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlLinkedNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
