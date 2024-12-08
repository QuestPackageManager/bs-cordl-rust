#[cfg(feature = "Newtonsoft+Json+Utilities+EnumInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumInfo {
    __cordl_parent: crate::System::Object,
    pub IsFlags: bool,
    pub Values: *mut quest_hook::libil2cpp::Il2CppArray<u64>,
    pub Names: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub ResolvedNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+EnumInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::EnumInfo =>
    "Newtonsoft.Json.Utilities"."EnumInfo"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+EnumInfo")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::EnumInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+EnumInfo")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::EnumInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+EnumInfo")]
impl crate::Newtonsoft::Json::Utilities::EnumInfo {
    pub fn _ctor(
        &mut self,
        isFlags: bool,
        values: *mut quest_hook::libil2cpp::Il2CppArray<u64>,
        names: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        resolvedNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (isFlags, values, names, resolvedNames))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        isFlags: bool,
        values: *mut quest_hook::libil2cpp::Il2CppArray<u64>,
        names: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        resolvedNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (isFlags, values, names, resolvedNames))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+EnumInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Utilities::EnumInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
