#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerObjectInfoInit")]
#[repr(C)]
#[derive(Debug)]
pub struct SerObjectInfoInit {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub seenBeforeTable: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Hashtable,
    >,
    pub objectInfoIdCount: i32,
    pub oiPool: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::Formatters::Binary::SerStack,
    >,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerObjectInfoInit")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit =>
    "System.Runtime.Serialization.Formatters.Binary"."SerObjectInfoInit"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerObjectInfoInit")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerObjectInfoInit")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerObjectInfoInit")]
impl crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit {
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
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerObjectInfoInit")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
