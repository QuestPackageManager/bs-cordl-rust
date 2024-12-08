#[cfg(feature = "System+Runtime+Serialization+SerializationInfoEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct SerializationInfoEnumerator {
    __cordl_parent: crate::System::Object,
    pub _members: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub _data: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    pub _types: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    pub _numItems: i32,
    pub _currItem: i32,
    pub _current: bool,
}
#[cfg(feature = "System+Runtime+Serialization+SerializationInfoEnumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::SerializationInfoEnumerator =>
    "System.Runtime.Serialization"."SerializationInfoEnumerator"
);
#[cfg(feature = "System+Runtime+Serialization+SerializationInfoEnumerator")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::SerializationInfoEnumerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SerializationInfoEnumerator")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::SerializationInfoEnumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SerializationInfoEnumerator")]
impl crate::System::Runtime::Serialization::SerializationInfoEnumerator {
    pub fn _ctor(
        &mut self,
        members: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        info: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
        types: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        numItems: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (members, info, types, numItems))?;
        Ok(__cordl_ret)
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Serialization::SerializationEntry,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Runtime::Serialization::SerializationEntry = __cordl_object
            .invoke("get_Current", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ObjectType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Value", ())?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("System.Collections.IEnumerator.get_Current", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        members: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        info: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
        types: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        numItems: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (members, info, types, numItems))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Serialization+SerializationInfoEnumerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::SerializationInfoEnumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
