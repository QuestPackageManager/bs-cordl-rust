#[cfg(feature = "System+Xml+XmlNodeChangedEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlNodeChangedEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub action: crate::System::Xml::XmlNodeChangedAction,
    pub node: *mut crate::System::Xml::XmlNode,
    pub oldParent: *mut crate::System::Xml::XmlNode,
    pub newParent: *mut crate::System::Xml::XmlNode,
    pub oldValue: *mut crate::System::String,
    pub newValue: *mut crate::System::String,
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
    pub fn _ctor(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
        oldParent: *mut crate::System::Xml::XmlNode,
        newParent: *mut crate::System::Xml::XmlNode,
        oldValue: *mut crate::System::String,
        newValue: *mut crate::System::String,
        action: crate::System::Xml::XmlNodeChangedAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (node, oldParent, newParent, oldValue, newValue, action))?;
        Ok(__cordl_ret)
    }
    pub fn get_Action(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlNodeChangedAction> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlNodeChangedAction = __cordl_object
            .invoke("get_Action", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        node: *mut crate::System::Xml::XmlNode,
        oldParent: *mut crate::System::Xml::XmlNode,
        newParent: *mut crate::System::Xml::XmlNode,
        oldValue: *mut crate::System::String,
        newValue: *mut crate::System::String,
        action: crate::System::Xml::XmlNodeChangedAction,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (node, oldParent, newParent, oldValue, newValue, action),
            )?;
        Ok(__cordl_object)
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
