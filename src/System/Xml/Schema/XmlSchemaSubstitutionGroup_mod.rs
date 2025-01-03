#[cfg(feature = "System+Xml+Schema+XmlSchemaSubstitutionGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaSubstitutionGroup {
    __cordl_parent: crate::System::Xml::Schema::XmlSchemaObject,
    pub membersList: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    pub examplar: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaSubstitutionGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaSubstitutionGroup
    => "System.Xml.Schema"."XmlSchemaSubstitutionGroup"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaSubstitutionGroup")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaSubstitutionGroup {
    type Target = crate::System::Xml::Schema::XmlSchemaObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaSubstitutionGroup")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlSchemaSubstitutionGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaSubstitutionGroup")]
impl crate::System::Xml::Schema::XmlSchemaSubstitutionGroup {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Examplar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlQualifiedName,
        > = __cordl_object.invoke("get_Examplar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Members(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ArrayList,
        > = __cordl_object.invoke("get_Members", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Examplar(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Examplar", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaSubstitutionGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XmlSchemaSubstitutionGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
