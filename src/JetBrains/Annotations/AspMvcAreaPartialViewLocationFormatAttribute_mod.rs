#[cfg(feature = "JetBrains+Annotations+AspMvcAreaPartialViewLocationFormatAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AspMvcAreaPartialViewLocationFormatAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Format_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "JetBrains+Annotations+AspMvcAreaPartialViewLocationFormatAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::JetBrains::Annotations::AspMvcAreaPartialViewLocationFormatAttribute =>
    "JetBrains.Annotations"."AspMvcAreaPartialViewLocationFormatAttribute"
);
#[cfg(feature = "JetBrains+Annotations+AspMvcAreaPartialViewLocationFormatAttribute")]
impl std::ops::Deref
for crate::JetBrains::Annotations::AspMvcAreaPartialViewLocationFormatAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+AspMvcAreaPartialViewLocationFormatAttribute")]
impl std::ops::DerefMut
for crate::JetBrains::Annotations::AspMvcAreaPartialViewLocationFormatAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+AspMvcAreaPartialViewLocationFormatAttribute")]
impl crate::JetBrains::Annotations::AspMvcAreaPartialViewLocationFormatAttribute {
    pub fn New(
        format: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (format))?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "JetBrains+Annotations+AspMvcAreaPartialViewLocationFormatAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::JetBrains::Annotations::AspMvcAreaPartialViewLocationFormatAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
