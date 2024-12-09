#[cfg(feature = "System+Reflection+AssemblyProductAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AssemblyProductAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Product_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Reflection+AssemblyProductAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::AssemblyProductAttribute =>
    "System.Reflection"."AssemblyProductAttribute"
);
#[cfg(feature = "System+Reflection+AssemblyProductAttribute")]
impl std::ops::Deref for crate::System::Reflection::AssemblyProductAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyProductAttribute")]
impl std::ops::DerefMut for crate::System::Reflection::AssemblyProductAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyProductAttribute")]
impl crate::System::Reflection::AssemblyProductAttribute {
    pub fn New(
        product: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (product))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        product: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (product))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Reflection+AssemblyProductAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::AssemblyProductAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
