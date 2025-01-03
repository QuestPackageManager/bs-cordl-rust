#[cfg(feature = "System+Xml+Schema+RestrictionFacets")]
#[repr(C)]
#[derive(Debug)]
pub struct RestrictionFacets {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Length: i32,
    pub MinLength: i32,
    pub MaxLength: i32,
    pub Patterns: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    pub Enumeration: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    pub WhiteSpace: crate::System::Xml::Schema::XmlSchemaWhiteSpace,
    pub MaxInclusive: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub MaxExclusive: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub MinInclusive: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub MinExclusive: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub TotalDigits: i32,
    pub FractionDigits: i32,
    pub Flags: crate::System::Xml::Schema::RestrictionFlags,
    pub FixedFlags: crate::System::Xml::Schema::RestrictionFlags,
}
#[cfg(feature = "System+Xml+Schema+RestrictionFacets")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::RestrictionFacets =>
    "System.Xml.Schema"."RestrictionFacets"
);
#[cfg(feature = "System+Xml+Schema+RestrictionFacets")]
impl std::ops::Deref for crate::System::Xml::Schema::RestrictionFacets {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+RestrictionFacets")]
impl std::ops::DerefMut for crate::System::Xml::Schema::RestrictionFacets {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+RestrictionFacets")]
impl crate::System::Xml::Schema::RestrictionFacets {
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
}
#[cfg(feature = "System+Xml+Schema+RestrictionFacets")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::RestrictionFacets {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
