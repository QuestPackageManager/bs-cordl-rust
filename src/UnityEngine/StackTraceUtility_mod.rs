#[cfg(feature = "UnityEngine+StackTraceUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct StackTraceUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+StackTraceUtility")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::StackTraceUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "StackTraceUtility";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+StackTraceUtility")]
impl std::ops::Deref for crate::UnityEngine::StackTraceUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+StackTraceUtility")]
impl std::ops::DerefMut for crate::UnityEngine::StackTraceUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+StackTraceUtility")]
impl crate::UnityEngine::StackTraceUtility {
    pub fn ExtractFormattedStackTrace(
        stackTrace: quest_hook::libil2cpp::Gc<crate::System::Diagnostics::StackTrace>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractFormattedStackTrace", (stackTrace))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractStackTrace() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractStackTrace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractStringFromExceptionInternal(
        exceptiono: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        message: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        stackTrace: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ExtractStringFromExceptionInternal",
                (exceptiono, message, stackTrace),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetProjectFolder(
        folder: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetProjectFolder", (folder))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+StackTraceUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::StackTraceUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
