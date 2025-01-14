#[cfg(feature = "UIExtensionMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct UIExtensionMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UIExtensionMethods")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::UIExtensionMethods {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "UIExtensionMethods";
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
#[cfg(feature = "UIExtensionMethods")]
impl std::ops::Deref for crate::GlobalNamespace::UIExtensionMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UIExtensionMethods")]
impl std::ops::DerefMut for crate::GlobalNamespace::UIExtensionMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UIExtensionMethods")]
impl crate::GlobalNamespace::UIExtensionMethods {
    pub fn CopySizeAndPositionFrom(
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CopySizeAndPositionFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CopySizeAndPositionFrom", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (target, source))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UIExtensionMethods")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::UIExtensionMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
