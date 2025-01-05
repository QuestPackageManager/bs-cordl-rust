#[cfg(feature = "System+Data+SqlTypes+SqlStreamChars")]
#[repr(C)]
#[derive(Debug)]
pub struct SqlStreamChars {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Data+SqlTypes+SqlStreamChars")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::SqlTypes::SqlStreamChars =>
    "System.Data.SqlTypes"."SqlStreamChars"
);
#[cfg(feature = "System+Data+SqlTypes+SqlStreamChars")]
impl std::ops::Deref for crate::System::Data::SqlTypes::SqlStreamChars {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlStreamChars")]
impl std::ops::DerefMut for crate::System::Data::SqlTypes::SqlStreamChars {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlStreamChars")]
impl crate::System::Data::SqlTypes::SqlStreamChars {
    pub fn Read(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Read", (buffer, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Seek(
        &mut self,
        offset: i64,
        origin: crate::System::IO::SeekOrigin,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("Seek", (offset, origin))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_Length", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Position(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_Position", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlStreamChars")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Data::SqlTypes::SqlStreamChars {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
