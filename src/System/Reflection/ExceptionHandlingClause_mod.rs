#[cfg(feature = "System+Reflection+ExceptionHandlingClause")]
#[repr(C)]
#[derive(Debug)]
pub struct ExceptionHandlingClause {
    __cordl_parent: crate::System::Object,
    pub catch_type: *mut crate::System::Type,
    pub filter_offset: i32,
    pub flags: crate::System::Reflection::ExceptionHandlingClauseOptions,
    pub try_offset: i32,
    pub try_length: i32,
    pub handler_offset: i32,
    pub handler_length: i32,
}
#[cfg(feature = "System+Reflection+ExceptionHandlingClause")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::ExceptionHandlingClause =>
    "System.Reflection"."ExceptionHandlingClause"
);
#[cfg(feature = "System+Reflection+ExceptionHandlingClause")]
impl std::ops::Deref for crate::System::Reflection::ExceptionHandlingClause {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+ExceptionHandlingClause")]
impl std::ops::DerefMut for crate::System::Reflection::ExceptionHandlingClause {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+ExceptionHandlingClause")]
impl crate::System::Reflection::ExceptionHandlingClause {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
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
#[cfg(feature = "System+Reflection+ExceptionHandlingClause")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::ExceptionHandlingClause {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
