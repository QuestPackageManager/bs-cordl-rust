#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerStack")]
#[repr(C)]
#[derive(Debug)]
pub struct SerStack {
    __cordl_parent: crate::System::Object,
    pub objects: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    pub stackId: *mut crate::System::String,
    pub top: i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerStack")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::SerStack =>
    "System.Runtime.Serialization.Formatters.Binary"."SerStack"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerStack")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::SerStack {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerStack")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::SerStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerStack")]
impl crate::System::Runtime::Serialization::Formatters::Binary::SerStack {
    pub fn IncreaseCapacity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncreaseCapacity", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEmpty", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        stackId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stackId))?;
        Ok(__cordl_object)
    }
    pub fn Peek(&mut self) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object.invoke("Peek", ())?;
        Ok(__cordl_ret)
    }
    pub fn PeekPeek(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("PeekPeek", ())?;
        Ok(__cordl_ret)
    }
    pub fn Pop(&mut self) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object.invoke("Pop", ())?;
        Ok(__cordl_ret)
    }
    pub fn Push(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Push", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        stackId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stackId))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerStack")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::SerStack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}