#[cfg(feature = "System+PasteArguments")]
#[repr(C)]
#[derive(Debug)]
pub struct PasteArguments {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+PasteArguments")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::PasteArguments => "System"
    ."PasteArguments"
);
#[cfg(feature = "System+PasteArguments")]
impl std::ops::Deref for crate::System::PasteArguments {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+PasteArguments")]
impl std::ops::DerefMut for crate::System::PasteArguments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+PasteArguments")]
impl crate::System::PasteArguments {
    pub fn AppendArgument(
        stringBuilder: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        argument: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendArgument", (stringBuilder, argument))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsNoWhitespaceOrQuotes(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsNoWhitespaceOrQuotes", (s))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+PasteArguments")]
impl quest_hook::libil2cpp::ObjectType for crate::System::PasteArguments {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
