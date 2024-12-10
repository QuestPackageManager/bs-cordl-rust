#[cfg(feature = "System+Xml+Serialization+UnreferencedObjectEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct UnreferencedObjectEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub o: *mut quest_hook::libil2cpp::Il2CppObject,
    pub id: *mut quest_hook::libil2cpp::Il2CppString,
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
    pub fn New(
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (o, id))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (o, id))?;
        Ok(__cordl_ret.into())
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
