#[cfg(feature = "System+Diagnostics+DebuggerDisplayAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DebuggerDisplayAttribute {
    __cordl_parent: crate::System::Attribute,
    pub name: *mut crate::System::String,
    pub value: *mut crate::System::String,
    pub _cordl_type: *mut crate::System::String,
}
#[cfg(feature = "System+Diagnostics+DebuggerDisplayAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::DebuggerDisplayAttribute =>
    "System.Diagnostics"."DebuggerDisplayAttribute"
);
#[cfg(feature = "System+Diagnostics+DebuggerDisplayAttribute")]
impl std::ops::Deref for crate::System::Diagnostics::DebuggerDisplayAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+DebuggerDisplayAttribute")]
impl std::ops::DerefMut for crate::System::Diagnostics::DebuggerDisplayAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+DebuggerDisplayAttribute")]
impl crate::System::Diagnostics::DebuggerDisplayAttribute {
    pub fn New(
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Name(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Name", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Type(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Type", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Diagnostics+DebuggerDisplayAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::DebuggerDisplayAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
