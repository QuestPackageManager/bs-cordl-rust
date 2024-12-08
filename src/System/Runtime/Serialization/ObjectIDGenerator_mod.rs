#[cfg(feature = "System+Runtime+Serialization+ObjectIDGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectIDGenerator {
    __cordl_parent: crate::System::Object,
    pub m_currentCount: i32,
    pub m_currentSize: i32,
    pub m_ids: *mut quest_hook::libil2cpp::Il2CppArray<i64>,
    pub m_objs: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
}
#[cfg(feature = "System+Runtime+Serialization+ObjectIDGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::ObjectIDGenerator => "System.Runtime.Serialization"
    ."ObjectIDGenerator"
);
#[cfg(feature = "System+Runtime+Serialization+ObjectIDGenerator")]
impl std::ops::Deref for crate::System::Runtime::Serialization::ObjectIDGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+ObjectIDGenerator")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::ObjectIDGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+ObjectIDGenerator")]
impl crate::System::Runtime::Serialization::ObjectIDGenerator {
    pub fn FindElement(
        &mut self,
        obj: *mut crate::System::Object,
        found: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindElement", (obj, found))?;
        Ok(__cordl_ret)
    }
    pub fn GetId(
        &mut self,
        obj: *mut crate::System::Object,
        firstTime: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetId", (obj, firstTime))?;
        Ok(__cordl_ret)
    }
    pub fn HasId(
        &mut self,
        obj: *mut crate::System::Object,
        firstTime: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("HasId", (obj, firstTime))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Rehash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Rehash", ())?;
        Ok(__cordl_ret)
    }
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
}
#[cfg(feature = "System+Runtime+Serialization+ObjectIDGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::ObjectIDGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}