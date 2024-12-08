#[cfg(feature = "System+Diagnostics+Switch")]
#[repr(C)]
#[derive(Debug)]
pub struct Switch {
    __cordl_parent: crate::System::Object,
    pub description: *mut crate::System::String,
    pub displayName: *mut crate::System::String,
    pub switchValueString: *mut crate::System::String,
    pub defaultValue: *mut crate::System::String,
}
#[cfg(feature = "System+Diagnostics+Switch")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::Switch =>
    "System.Diagnostics"."Switch"
);
#[cfg(feature = "System+Diagnostics+Switch")]
impl std::ops::Deref for crate::System::Diagnostics::Switch {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+Switch")]
impl std::ops::DerefMut for crate::System::Diagnostics::Switch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+Switch")]
impl crate::System::Diagnostics::Switch {
    pub fn _ctor_String_String0(
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
    pub fn _ctor_String1(
        &mut self,
        displayName: *mut crate::System::String,
        description: *mut crate::System::String,
        defaultSwitchValue: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (displayName, description, defaultSwitchValue))?;
        Ok(__cordl_ret)
    }
    pub fn New_String_String0(
        displayName: *mut crate::System::String,
        description: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (displayName, description))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        displayName: *mut crate::System::String,
        description: *mut crate::System::String,
        defaultSwitchValue: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (displayName, description, defaultSwitchValue))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Diagnostics+Switch")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Diagnostics::Switch {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
