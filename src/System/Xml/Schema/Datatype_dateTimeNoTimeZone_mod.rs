#[cfg(feature = "System+Xml+Schema+Datatype_dateTimeNoTimeZone")]
#[repr(C)]
#[derive(Debug)]
pub struct Datatype_dateTimeNoTimeZone {
    __cordl_parent: crate::System::Xml::Schema::Datatype_dateTimeBase,
}
#[cfg(feature = "System+Xml+Schema+Datatype_dateTimeNoTimeZone")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Datatype_dateTimeNoTimeZone
    => "System.Xml.Schema"."Datatype_dateTimeNoTimeZone"
);
#[cfg(feature = "System+Xml+Schema+Datatype_dateTimeNoTimeZone")]
impl std::ops::Deref for crate::System::Xml::Schema::Datatype_dateTimeNoTimeZone {
    type Target = crate::System::Xml::Schema::Datatype_dateTimeBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_dateTimeNoTimeZone")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Datatype_dateTimeNoTimeZone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_dateTimeNoTimeZone")]
impl crate::System::Xml::Schema::Datatype_dateTimeNoTimeZone {
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
#[cfg(feature = "System+Xml+Schema+Datatype_dateTimeNoTimeZone")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::Datatype_dateTimeNoTimeZone {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
