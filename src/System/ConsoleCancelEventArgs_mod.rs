#[cfg(feature = "System+ConsoleCancelEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct ConsoleCancelEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub _type: crate::System::ConsoleSpecialKey,
    pub _Cancel_k__BackingField: bool,
}
#[cfg(feature = "System+ConsoleCancelEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ConsoleCancelEventArgs => "System"
    ."ConsoleCancelEventArgs"
);
#[cfg(feature = "System+ConsoleCancelEventArgs")]
impl std::ops::Deref for crate::System::ConsoleCancelEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ConsoleCancelEventArgs")]
impl std::ops::DerefMut for crate::System::ConsoleCancelEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ConsoleCancelEventArgs")]
impl crate::System::ConsoleCancelEventArgs {
    pub fn New_1() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_ConsoleSpecialKey0(
        _cordl_type: crate::System::ConsoleSpecialKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ConsoleSpecialKey0(
        &mut self,
        _cordl_type: crate::System::ConsoleSpecialKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Cancel(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Cancel", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ConsoleCancelEventArgs")]
impl quest_hook::libil2cpp::ObjectType for crate::System::ConsoleCancelEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
