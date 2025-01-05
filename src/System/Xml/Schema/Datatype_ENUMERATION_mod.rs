#[cfg(feature = "System+Xml+Schema+Datatype_ENUMERATION")]
#[repr(C)]
#[derive(Debug)]
pub struct Datatype_ENUMERATION {
    __cordl_parent: crate::System::Xml::Schema::Datatype_NMTOKEN,
}
#[cfg(feature = "System+Xml+Schema+Datatype_ENUMERATION")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Datatype_ENUMERATION =>
    "System.Xml.Schema"."Datatype_ENUMERATION"
);
#[cfg(feature = "System+Xml+Schema+Datatype_ENUMERATION")]
impl std::ops::Deref for crate::System::Xml::Schema::Datatype_ENUMERATION {
    type Target = crate::System::Xml::Schema::Datatype_NMTOKEN;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_ENUMERATION")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Datatype_ENUMERATION {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_ENUMERATION")]
impl crate::System::Xml::Schema::Datatype_ENUMERATION {
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
    pub fn get_TokenizedType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlTokenizedType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlTokenizedType = __cordl_object
            .invoke("get_TokenizedType", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_ENUMERATION")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::Datatype_ENUMERATION {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
