#[cfg(feature = "System+Reflection+AssemblyDelaySignAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AssemblyDelaySignAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _DelaySign_k__BackingField: bool,
}
#[cfg(feature = "System+Reflection+AssemblyDelaySignAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Reflection::AssemblyDelaySignAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Reflection";
    const CLASS_NAME: &'static str = "AssemblyDelaySignAttribute";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Reflection+AssemblyDelaySignAttribute")]
impl std::ops::Deref for crate::System::Reflection::AssemblyDelaySignAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyDelaySignAttribute")]
impl std::ops::DerefMut for crate::System::Reflection::AssemblyDelaySignAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyDelaySignAttribute")]
impl crate::System::Reflection::AssemblyDelaySignAttribute {
    pub fn New(
        delaySign: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (delaySign))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        delaySign: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (delaySign))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Reflection+AssemblyDelaySignAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::AssemblyDelaySignAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
