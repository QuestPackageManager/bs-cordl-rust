#[cfg(feature = "System+Diagnostics+TraceSwitch")]
#[repr(C)]
#[derive(Debug)]
pub struct TraceSwitch {
    __cordl_parent: crate::System::Diagnostics::Switch,
}
#[cfg(feature = "System+Diagnostics+TraceSwitch")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::TraceSwitch =>
    "System.Diagnostics"."TraceSwitch"
);
#[cfg(feature = "System+Diagnostics+TraceSwitch")]
impl std::ops::Deref for crate::System::Diagnostics::TraceSwitch {
    type Target = crate::System::Diagnostics::Switch;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+TraceSwitch")]
impl std::ops::DerefMut for crate::System::Diagnostics::TraceSwitch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+TraceSwitch")]
impl crate::System::Diagnostics::TraceSwitch {
    pub fn New(
        displayName: *mut crate::System::String,
        description: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (displayName, description))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        displayName: *mut crate::System::String,
        description: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (displayName, description))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Diagnostics+TraceSwitch")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Diagnostics::TraceSwitch {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
