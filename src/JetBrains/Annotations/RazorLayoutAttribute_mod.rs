#[cfg(feature = "JetBrains+Annotations+RazorLayoutAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct RazorLayoutAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "JetBrains+Annotations+RazorLayoutAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::JetBrains::Annotations::RazorLayoutAttribute =>
    "JetBrains.Annotations"."RazorLayoutAttribute"
);
#[cfg(feature = "JetBrains+Annotations+RazorLayoutAttribute")]
impl std::ops::Deref for crate::JetBrains::Annotations::RazorLayoutAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+RazorLayoutAttribute")]
impl std::ops::DerefMut for crate::JetBrains::Annotations::RazorLayoutAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+RazorLayoutAttribute")]
impl crate::JetBrains::Annotations::RazorLayoutAttribute {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "JetBrains+Annotations+RazorLayoutAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::JetBrains::Annotations::RazorLayoutAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
