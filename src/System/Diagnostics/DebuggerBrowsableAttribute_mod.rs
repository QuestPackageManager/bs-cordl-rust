#[cfg(feature = "System+Diagnostics+DebuggerBrowsableAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DebuggerBrowsableAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    pub state: crate::System::Diagnostics::DebuggerBrowsableState,
}
#[cfg(feature = "System+Diagnostics+DebuggerBrowsableAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::DebuggerBrowsableAttribute
    => "System.Diagnostics"."DebuggerBrowsableAttribute"
);
#[cfg(feature = "System+Diagnostics+DebuggerBrowsableAttribute")]
impl std::ops::Deref for crate::System::Diagnostics::DebuggerBrowsableAttribute {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Attribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+DebuggerBrowsableAttribute")]
impl std::ops::DerefMut for crate::System::Diagnostics::DebuggerBrowsableAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+DebuggerBrowsableAttribute")]
impl crate::System::Diagnostics::DebuggerBrowsableAttribute {
    pub fn New(
        state: crate::System::Diagnostics::DebuggerBrowsableState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (state))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        state: crate::System::Diagnostics::DebuggerBrowsableState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (state))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Diagnostics+DebuggerBrowsableAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::DebuggerBrowsableAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
