#[cfg(feature = "System+Xml+Schema+Datatype_dateTimeTimeZone")]
#[repr(C)]
#[derive(Debug)]
pub struct Datatype_dateTimeTimeZone {
    __cordl_parent: crate::System::Xml::Schema::Datatype_dateTimeBase,
}
#[cfg(feature = "System+Xml+Schema+Datatype_dateTimeTimeZone")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Datatype_dateTimeTimeZone
    => "System.Xml.Schema"."Datatype_dateTimeTimeZone"
);
#[cfg(feature = "System+Xml+Schema+Datatype_dateTimeTimeZone")]
impl std::ops::Deref for crate::System::Xml::Schema::Datatype_dateTimeTimeZone {
    type Target = crate::System::Xml::Schema::Datatype_dateTimeBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_dateTimeTimeZone")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Datatype_dateTimeTimeZone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_dateTimeTimeZone")]
impl crate::System::Xml::Schema::Datatype_dateTimeTimeZone {
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
#[cfg(feature = "System+Xml+Schema+Datatype_dateTimeTimeZone")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::Datatype_dateTimeTimeZone {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
