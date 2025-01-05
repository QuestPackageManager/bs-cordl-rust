#[cfg(feature = "Newtonsoft+Json+Converters+XDeclarationWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct XDeclarationWrapper {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Converters::XObjectWrapper,
    >,
    pub _Declaration_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Linq::XDeclaration,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDeclarationWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Converters::XDeclarationWrapper => "Newtonsoft.Json.Converters"
    ."XDeclarationWrapper"
);
#[cfg(feature = "Newtonsoft+Json+Converters+XDeclarationWrapper")]
impl std::ops::Deref for crate::Newtonsoft::Json::Converters::XDeclarationWrapper {
    type Target = quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Converters::XObjectWrapper,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDeclarationWrapper")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Converters::XDeclarationWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDeclarationWrapper")]
impl crate::Newtonsoft::Json::Converters::XDeclarationWrapper {
    pub fn New(
        declaration: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XDeclaration>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (declaration))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        declaration: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XDeclaration>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (declaration))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Declaration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XDeclaration>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Linq::XDeclaration,
        > = __cordl_object.invoke("get_Declaration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Encoding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Encoding", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NodeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlNodeType = __cordl_object
            .invoke("get_NodeType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Standalone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Standalone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Version", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Encoding(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Encoding", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Standalone(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Standalone", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDeclarationWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::XDeclarationWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDeclarationWrapper")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlDeclaration>,
> for crate::Newtonsoft::Json::Converters::XDeclarationWrapper {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Converters::IXmlDeclaration,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDeclarationWrapper")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlDeclaration>,
> for crate::Newtonsoft::Json::Converters::XDeclarationWrapper {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Converters::IXmlDeclaration,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDeclarationWrapper")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>>
for crate::Newtonsoft::Json::Converters::XDeclarationWrapper {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XDeclarationWrapper")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>>
for crate::Newtonsoft::Json::Converters::XDeclarationWrapper {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode> {
        unsafe { std::mem::transmute(self) }
    }
}
