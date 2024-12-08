#[cfg(feature = "System+Dynamic+IDynamicMetaObjectProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IDynamicMetaObjectProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Dynamic+IDynamicMetaObjectProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::IDynamicMetaObjectProvider =>
    "System.Dynamic"."IDynamicMetaObjectProvider"
);
#[cfg(feature = "System+Dynamic+IDynamicMetaObjectProvider")]
impl std::ops::Deref for crate::System::Dynamic::IDynamicMetaObjectProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+IDynamicMetaObjectProvider")]
impl std::ops::DerefMut for crate::System::Dynamic::IDynamicMetaObjectProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+IDynamicMetaObjectProvider")]
impl crate::System::Dynamic::IDynamicMetaObjectProvider {
    pub fn GetMetaObject(
        &mut self,
        parameter: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("GetMetaObject", (parameter))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Dynamic+IDynamicMetaObjectProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::IDynamicMetaObjectProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
