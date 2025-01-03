#[cfg(feature = "System+Xml+XmlNodeChangedEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlNodeChangedEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub action: crate::System::Xml::XmlNodeChangedAction,
    pub node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    pub oldParent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    pub newParent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    pub oldValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub newValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Xml+XmlNodeChangedEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlNodeChangedEventArgs =>
    "System.Xml"."XmlNodeChangedEventArgs"
);
#[cfg(feature = "System+Xml+XmlNodeChangedEventArgs")]
impl std::ops::Deref for crate::System::Xml::XmlNodeChangedEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlNodeChangedEventArgs")]
impl std::ops::DerefMut for crate::System::Xml::XmlNodeChangedEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlNodeChangedEventArgs")]
impl crate::System::Xml::XmlNodeChangedEventArgs {
    pub fn New(
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        oldParent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        newParent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        oldValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        action: crate::System::Xml::XmlNodeChangedAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (node, oldParent, newParent, oldValue, newValue, action),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        oldParent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        newParent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        oldValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        action: crate::System::Xml::XmlNodeChangedAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (node, oldParent, newParent, oldValue, newValue, action))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Action(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlNodeChangedAction> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlNodeChangedAction = __cordl_object
            .invoke("get_Action", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlNodeChangedEventArgs")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlNodeChangedEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
