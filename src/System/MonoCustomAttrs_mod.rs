#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoCustomAttrs_AttributeInfo {
    __cordl_parent: crate::System::Object,
    pub _usage: *mut crate::System::AttributeUsageAttribute,
    pub _inheritanceLevel: i32,
}
#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::MonoCustomAttrs_AttributeInfo =>
    "System"."MonoCustomAttrs/AttributeInfo"
);
#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
impl std::ops::Deref for crate::System::MonoCustomAttrs_AttributeInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
impl std::ops::DerefMut for crate::System::MonoCustomAttrs_AttributeInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
impl crate::System::MonoCustomAttrs_AttributeInfo {
    pub fn get_InheritanceLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_InheritanceLevel", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        usage: *mut crate::System::AttributeUsageAttribute,
        inheritanceLevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (usage, inheritanceLevel))?;
        Ok(__cordl_ret)
    }
    pub fn get_Usage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::AttributeUsageAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::AttributeUsageAttribute = __cordl_object
            .invoke("get_Usage", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        usage: *mut crate::System::AttributeUsageAttribute,
        inheritanceLevel: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (usage, inheritanceLevel))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::MonoCustomAttrs_AttributeInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+MonoCustomAttrs")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoCustomAttrs {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+MonoCustomAttrs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::MonoCustomAttrs => "System"
    ."MonoCustomAttrs"
);
#[cfg(feature = "System+MonoCustomAttrs")]
impl std::ops::Deref for crate::System::MonoCustomAttrs {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoCustomAttrs")]
impl std::ops::DerefMut for crate::System::MonoCustomAttrs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoCustomAttrs")]
impl crate::System::MonoCustomAttrs {
    #[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
    pub type AttributeInfo = crate::System::MonoCustomAttrs_AttributeInfo;
}
#[cfg(feature = "System+MonoCustomAttrs")]
impl quest_hook::libil2cpp::ObjectType for crate::System::MonoCustomAttrs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
