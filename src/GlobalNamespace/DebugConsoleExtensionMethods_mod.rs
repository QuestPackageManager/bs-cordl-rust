#[cfg(feature = "cordl_class_DebugConsoleExtensionMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct DebugConsoleExtensionMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_DebugConsoleExtensionMethods")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::DebugConsoleExtensionMethods {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "DebugConsoleExtensionMethods";
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
#[cfg(feature = "DebugConsoleExtensionMethods")]
impl std::ops::Deref for crate::GlobalNamespace::DebugConsoleExtensionMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DebugConsoleExtensionMethods")]
impl std::ops::DerefMut for crate::GlobalNamespace::DebugConsoleExtensionMethods {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DebugConsoleExtensionMethods")]
impl crate::GlobalNamespace::DebugConsoleExtensionMethods {
    pub fn ToConsoleMessage(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: crate::UnityEngine::LogType,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::LogType,
                        ),
                        crate::GlobalNamespace::DebugConsoleController_ConsoleMessage,
                        2usize,
                    >("ToConsoleMessage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToConsoleMessage", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::DebugConsoleController_ConsoleMessage = unsafe {
            cordl_method_info.invoke_unchecked((), (message, _cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_DebugConsoleExtensionMethods")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DebugConsoleExtensionMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
