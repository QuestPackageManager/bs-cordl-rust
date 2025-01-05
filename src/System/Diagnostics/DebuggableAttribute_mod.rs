#[cfg(feature = "System+Diagnostics+DebuggableAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DebuggableAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    pub m_debuggingModes: crate::System::Diagnostics::DebuggableAttribute_DebuggingModes,
}
#[cfg(feature = "System+Diagnostics+DebuggableAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::DebuggableAttribute =>
    "System.Diagnostics"."DebuggableAttribute"
);
#[cfg(feature = "System+Diagnostics+DebuggableAttribute")]
impl std::ops::Deref for crate::System::Diagnostics::DebuggableAttribute {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Attribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+DebuggableAttribute")]
impl std::ops::DerefMut for crate::System::Diagnostics::DebuggableAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+DebuggableAttribute")]
impl crate::System::Diagnostics::DebuggableAttribute {
    #[cfg(feature = "System+Diagnostics+DebuggableAttribute+DebuggingModes")]
    pub type DebuggingModes = crate::System::Diagnostics::DebuggableAttribute_DebuggingModes;
    pub fn New(
        modes: crate::System::Diagnostics::DebuggableAttribute_DebuggingModes,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (modes))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        modes: crate::System::Diagnostics::DebuggableAttribute_DebuggingModes,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (modes))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Diagnostics+DebuggableAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::DebuggableAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Diagnostics+DebuggableAttribute+DebuggingModes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DebuggableAttribute_DebuggingModes {
    #[default]
    Default = 1i32,
    DisableOptimizations = 256i32,
    EnableEditAndContinue = 4i32,
    IgnoreSymbolStoreSequencePoints = 2i32,
    None = 0i32,
}
#[cfg(feature = "System+Diagnostics+DebuggableAttribute+DebuggingModes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Diagnostics::DebuggableAttribute_DebuggingModes => "System.Diagnostics"
    ."DebuggableAttribute/DebuggingModes"
);
