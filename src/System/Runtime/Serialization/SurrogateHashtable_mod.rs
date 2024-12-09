#[cfg(feature = "System+Runtime+Serialization+SurrogateHashtable")]
#[repr(C)]
#[derive(Debug)]
pub struct SurrogateHashtable {
    __cordl_parent: crate::System::Collections::Hashtable,
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateHashtable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::SurrogateHashtable =>
    "System.Runtime.Serialization"."SurrogateHashtable"
);
#[cfg(feature = "System+Runtime+Serialization+SurrogateHashtable")]
impl std::ops::Deref for crate::System::Runtime::Serialization::SurrogateHashtable {
    type Target = crate::System::Collections::Hashtable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateHashtable")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::SurrogateHashtable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateHashtable")]
impl crate::System::Runtime::Serialization::SurrogateHashtable {
    pub fn KeyEquals(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppObject,
        item: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("KeyEquals", (key, item))?;
        Ok(__cordl_ret)
    }
    pub fn New(_cordl_size: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_size))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_size))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateHashtable")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::SurrogateHashtable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
