#[cfg(feature = "System+Data+SqlTypes+INullable")]
#[repr(C)]
#[derive(Debug)]
pub struct INullable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Data+SqlTypes+INullable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::SqlTypes::INullable =>
    "System.Data.SqlTypes"."INullable"
);
#[cfg(feature = "System+Data+SqlTypes+INullable")]
impl std::ops::Deref for crate::System::Data::SqlTypes::INullable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+SqlTypes+INullable")]
impl std::ops::DerefMut for crate::System::Data::SqlTypes::INullable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+SqlTypes+INullable")]
impl crate::System::Data::SqlTypes::INullable {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsNull", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+SqlTypes+INullable")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::SqlTypes::INullable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
