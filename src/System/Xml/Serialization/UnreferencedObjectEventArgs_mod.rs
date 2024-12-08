#[cfg(feature = "System+Xml+Serialization+UnreferencedObjectEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct UnreferencedObjectEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub o: *mut crate::System::Object,
    pub id: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+Serialization+UnreferencedObjectEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::UnreferencedObjectEventArgs =>
    "System.Xml.Serialization"."UnreferencedObjectEventArgs"
);
#[cfg(feature = "System+Xml+Serialization+UnreferencedObjectEventArgs")]
impl std::ops::Deref for crate::System::Xml::Serialization::UnreferencedObjectEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+UnreferencedObjectEventArgs")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::UnreferencedObjectEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+UnreferencedObjectEventArgs")]
impl crate::System::Xml::Serialization::UnreferencedObjectEventArgs {
    pub fn _ctor(
        &mut self,
        o: *mut crate::System::Object,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (o, id))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        o: *mut crate::System::Object,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (o, id))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Serialization+UnreferencedObjectEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::UnreferencedObjectEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
