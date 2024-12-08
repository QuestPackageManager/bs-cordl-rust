#[cfg(feature = "System+Xml+Schema+XmlSchemaGroupRef")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaGroupRef {
    __cordl_parent: crate::System::Xml::Schema::XmlSchemaParticle,
    pub refName: *mut crate::System::Xml::XmlQualifiedName,
    pub particle: *mut crate::System::Xml::Schema::XmlSchemaGroupBase,
    pub refined: *mut crate::System::Xml::Schema::XmlSchemaGroup,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaGroupRef")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaGroupRef =>
    "System.Xml.Schema"."XmlSchemaGroupRef"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaGroupRef")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaGroupRef {
    type Target = crate::System::Xml::Schema::XmlSchemaParticle;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaGroupRef")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlSchemaGroupRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaGroupRef")]
impl crate::System::Xml::Schema::XmlSchemaGroupRef {
    pub fn get_RefName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlQualifiedName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlQualifiedName = __cordl_object
            .invoke("get_RefName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Particle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaGroupBase,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaGroupBase = __cordl_object
            .invoke("get_Particle", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Redefined(
        &mut self,
        value: *mut crate::System::Xml::Schema::XmlSchemaGroup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Redefined", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetParticle(
        &mut self,
        value: *mut crate::System::Xml::Schema::XmlSchemaGroupBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetParticle", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Redefined(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchemaGroup> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaGroup = __cordl_object
            .invoke("get_Redefined", ())?;
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
    pub fn set_RefName(
        &mut self,
        value: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RefName", (value))?;
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
#[cfg(feature = "System+Xml+Schema+XmlSchemaGroupRef")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XmlSchemaGroupRef {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
