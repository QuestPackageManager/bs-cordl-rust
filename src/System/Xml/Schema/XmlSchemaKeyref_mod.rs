#[cfg(feature = "System+Xml+Schema+XmlSchemaKeyref")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaKeyref {
    __cordl_parent: crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
    pub refer: *mut crate::System::Xml::XmlQualifiedName,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaKeyref")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaKeyref =>
    "System.Xml.Schema"."XmlSchemaKeyref"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaKeyref")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaKeyref {
    type Target = crate::System::Xml::Schema::XmlSchemaIdentityConstraint;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaKeyref")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlSchemaKeyref {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaKeyref")]
impl crate::System::Xml::Schema::XmlSchemaKeyref {
    pub fn get_Refer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlQualifiedName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlQualifiedName = __cordl_object
            .invoke("get_Refer", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Refer(
        &mut self,
        value: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Refer", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaKeyref")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::XmlSchemaKeyref {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
