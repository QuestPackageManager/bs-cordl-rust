#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerObjectInfoCache")]
#[repr(C)]
#[derive(Debug)]
pub struct SerObjectInfoCache {
    __cordl_parent: crate::System::Object,
    pub fullTypeName: *mut crate::System::String,
    pub assemblyString: *mut crate::System::String,
    pub hasTypeForwardedFrom: bool,
    pub memberInfos: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Reflection::MemberInfo,
    >,
    pub memberNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub memberTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerObjectInfoCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoCache =>
    "System.Runtime.Serialization.Formatters.Binary"."SerObjectInfoCache"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerObjectInfoCache")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoCache {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerObjectInfoCache")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerObjectInfoCache")]
impl crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoCache {
    pub fn _ctor_String_String__cordl_bool0(
        &mut self,
        typeName: *mut crate::System::String,
        assemblyName: *mut crate::System::String,
        hasTypeForwardedFrom: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (typeName, assemblyName, hasTypeForwardedFrom))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Type1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn New_String_String__cordl_bool0(
        typeName: *mut crate::System::String,
        assemblyName: *mut crate::System::String,
        hasTypeForwardedFrom: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (typeName, assemblyName, hasTypeForwardedFrom))?;
        Ok(__cordl_object)
    }
    pub fn New_Type1(
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+SerObjectInfoCache")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
