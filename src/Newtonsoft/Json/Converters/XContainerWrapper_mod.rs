#[cfg(feature = "Newtonsoft+Json+Converters+XContainerWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct XContainerWrapper {
    __cordl_parent: crate::Newtonsoft::Json::Converters::XObjectWrapper,
    pub _childNodes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Converters+XContainerWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Converters::XContainerWrapper
    => "Newtonsoft.Json.Converters"."XContainerWrapper"
);
#[cfg(feature = "Newtonsoft+Json+Converters+XContainerWrapper")]
impl std::ops::Deref for crate::Newtonsoft::Json::Converters::XContainerWrapper {
    type Target = crate::Newtonsoft::Json::Converters::XObjectWrapper;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XContainerWrapper")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Converters::XContainerWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XContainerWrapper")]
impl crate::Newtonsoft::Json::Converters::XContainerWrapper {
    pub fn AppendChild(
        &mut self,
        newChild: *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlNode = __cordl_object
            .invoke("AppendChild", (newChild))?;
        Ok(__cordl_ret)
    }
    pub fn get_ChildNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        > = __cordl_object.invoke("get_ChildNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParentNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlNode = __cordl_object
            .invoke("get_ParentNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Container(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Linq::XContainer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Linq::XContainer = __cordl_object
            .invoke("get_Container", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        container: *mut crate::System::Xml::Linq::XContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (container))?;
        Ok(__cordl_ret)
    }
    pub fn get_HasChildNodes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasChildNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        container: *mut crate::System::Xml::Linq::XContainer,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XContainerWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::XContainerWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
