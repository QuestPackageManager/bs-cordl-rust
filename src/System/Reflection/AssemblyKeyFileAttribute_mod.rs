#[cfg(feature = "System+Reflection+AssemblyKeyFileAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AssemblyKeyFileAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _KeyFile_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "System+Reflection+AssemblyKeyFileAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::AssemblyKeyFileAttribute =>
    "System.Reflection"."AssemblyKeyFileAttribute"
);
#[cfg(feature = "System+Reflection+AssemblyKeyFileAttribute")]
impl std::ops::Deref for crate::System::Reflection::AssemblyKeyFileAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyKeyFileAttribute")]
impl std::ops::DerefMut for crate::System::Reflection::AssemblyKeyFileAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyKeyFileAttribute")]
impl crate::System::Reflection::AssemblyKeyFileAttribute {
    pub fn _ctor(
        &mut self,
        keyFile: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyFile))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        keyFile: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keyFile))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Reflection+AssemblyKeyFileAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::AssemblyKeyFileAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
