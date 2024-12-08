#[cfg(feature = "UnityEngine+InputSystem+Utilities+DisplayStringFormatAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DisplayStringFormatAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _formatString_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+DisplayStringFormatAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::DisplayStringFormatAttribute =>
    "UnityEngine.InputSystem.Utilities"."DisplayStringFormatAttribute"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+DisplayStringFormatAttribute")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Utilities::DisplayStringFormatAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+DisplayStringFormatAttribute")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Utilities::DisplayStringFormatAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+DisplayStringFormatAttribute")]
impl crate::UnityEngine::InputSystem::Utilities::DisplayStringFormatAttribute {
    pub fn New(
        formatString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (formatString))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        formatString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (formatString))?;
        Ok(__cordl_ret)
    }
    pub fn get_formatString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_formatString", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_formatString(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_formatString", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+DisplayStringFormatAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::DisplayStringFormatAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
