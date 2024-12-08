#[cfg(feature = "System+Text+RegularExpressions+RegexRunnerFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct RegexRunnerFactory {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexRunnerFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Text::RegularExpressions::RegexRunnerFactory =>
    "System.Text.RegularExpressions"."RegexRunnerFactory"
);
#[cfg(feature = "System+Text+RegularExpressions+RegexRunnerFactory")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::RegexRunnerFactory {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexRunnerFactory")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::RegexRunnerFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexRunnerFactory")]
impl crate::System::Text::RegularExpressions::RegexRunnerFactory {
    pub fn CreateInstance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexRunner,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexRunner = __cordl_object
            .invoke("CreateInstance", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexRunnerFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::RegexRunnerFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}