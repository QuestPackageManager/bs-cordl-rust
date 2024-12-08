#[cfg(feature = "JetBrains+Annotations+AspMvcPartialViewLocationFormatAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AspMvcPartialViewLocationFormatAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Format_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "JetBrains+Annotations+AspMvcPartialViewLocationFormatAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::JetBrains::Annotations::AspMvcPartialViewLocationFormatAttribute =>
    "JetBrains.Annotations"."AspMvcPartialViewLocationFormatAttribute"
);
#[cfg(feature = "JetBrains+Annotations+AspMvcPartialViewLocationFormatAttribute")]
impl std::ops::Deref
for crate::JetBrains::Annotations::AspMvcPartialViewLocationFormatAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+AspMvcPartialViewLocationFormatAttribute")]
impl std::ops::DerefMut
for crate::JetBrains::Annotations::AspMvcPartialViewLocationFormatAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+AspMvcPartialViewLocationFormatAttribute")]
impl crate::JetBrains::Annotations::AspMvcPartialViewLocationFormatAttribute {
    pub fn get_Format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Format", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        format: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (format))?;
        Ok(__cordl_ret)
    }
    pub fn set_Format(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Format", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        format: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (format))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "JetBrains+Annotations+AspMvcPartialViewLocationFormatAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::JetBrains::Annotations::AspMvcPartialViewLocationFormatAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
