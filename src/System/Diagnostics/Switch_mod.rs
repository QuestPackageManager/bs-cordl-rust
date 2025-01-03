#[cfg(feature = "System+Diagnostics+Switch")]
#[repr(C)]
#[derive(Debug)]
pub struct Switch {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub description: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub switchValueString: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub defaultValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Diagnostics+Switch")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::Switch =>
    "System.Diagnostics"."Switch"
);
#[cfg(feature = "System+Diagnostics+Switch")]
impl std::ops::Deref for crate::System::Diagnostics::Switch {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New_Il2CppString1(
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        description: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultSwitchValue: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (displayName, description, defaultSwitchValue))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Il2CppString0(
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        description: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (displayName, description))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        description: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultSwitchValue: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (displayName, description, defaultSwitchValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppString0(
        &mut self,
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        description: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (displayName, description))?;
        Ok(__cordl_ret.into())
    }
    pub fn _pruneCachedSwitches() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("_pruneCachedSwitches", ())?;
        Ok(__cordl_ret.into())
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
