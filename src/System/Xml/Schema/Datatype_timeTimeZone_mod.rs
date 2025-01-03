#[cfg(feature = "System+Xml+Schema+Datatype_timeTimeZone")]
#[repr(C)]
#[derive(Debug)]
pub struct Datatype_timeTimeZone {
    __cordl_parent: crate::System::Xml::Schema::Datatype_dateTimeBase,
}
#[cfg(feature = "System+Xml+Schema+Datatype_timeTimeZone")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Datatype_timeTimeZone =>
    "System.Xml.Schema"."Datatype_timeTimeZone"
);
#[cfg(feature = "System+Xml+Schema+Datatype_timeTimeZone")]
impl std::ops::Deref for crate::System::Xml::Schema::Datatype_timeTimeZone {
    type Target = crate::System::Xml::Schema::Datatype_dateTimeBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_timeTimeZone")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Datatype_timeTimeZone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_timeTimeZone")]
impl crate::System::Xml::Schema::Datatype_timeTimeZone {
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
#[cfg(feature = "System+Xml+Schema+Datatype_timeTimeZone")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::Datatype_timeTimeZone {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
