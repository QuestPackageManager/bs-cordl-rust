#[cfg(feature = "System+Reflection+RtFieldInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct RtFieldInfo {
    __cordl_parent: crate::System::Reflection::FieldInfo,
}
#[cfg(feature = "System+Reflection+RtFieldInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::RtFieldInfo =>
    "System.Reflection"."RtFieldInfo"
);
#[cfg(feature = "System+Reflection+RtFieldInfo")]
impl std::ops::Deref for crate::System::Reflection::RtFieldInfo {
    type Target = crate::System::Reflection::FieldInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+RtFieldInfo")]
impl std::ops::DerefMut for crate::System::Reflection::RtFieldInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+RtFieldInfo")]
impl crate::System::Reflection::RtFieldInfo {
    pub fn CheckConsistency(
        &mut self,
        target: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckConsistency", (target))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn UnsafeGetValue(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("UnsafeGetValue", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn UnsafeSetValue(
        &mut self,
        obj: *mut crate::System::Object,
        value: *mut crate::System::Object,
        invokeAttr: crate::System::Reflection::BindingFlags,
        binder: *mut crate::System::Reflection::Binder,
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnsafeSetValue", (obj, value, invokeAttr, binder, culture))?;
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
#[cfg(feature = "System+Reflection+RtFieldInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Reflection::RtFieldInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
