#[cfg(feature = "System+Xml+Schema+Datatype_nonPositiveInteger")]
#[repr(C)]
#[derive(Debug)]
pub struct Datatype_nonPositiveInteger {
    __cordl_parent: crate::System::Xml::Schema::Datatype_integer,
}
#[cfg(feature = "System+Xml+Schema+Datatype_nonPositiveInteger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Datatype_nonPositiveInteger
    => "System.Xml.Schema"."Datatype_nonPositiveInteger"
);
#[cfg(feature = "System+Xml+Schema+Datatype_nonPositiveInteger")]
impl std::ops::Deref for crate::System::Xml::Schema::Datatype_nonPositiveInteger {
    type Target = crate::System::Xml::Schema::Datatype_integer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_nonPositiveInteger")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Datatype_nonPositiveInteger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_nonPositiveInteger")]
impl crate::System::Xml::Schema::Datatype_nonPositiveInteger {
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
    pub fn get_FacetsChecker(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::FacetsChecker>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::FacetsChecker,
        > = __cordl_object.invoke("get_FacetsChecker", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasValueFacets(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasValueFacets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TypeCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::XmlTypeCode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlTypeCode = __cordl_object
            .invoke("get_TypeCode", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_nonPositiveInteger")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::Datatype_nonPositiveInteger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
