#[cfg(feature = "UnityEngine+CustomRenderTextureManager")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomRenderTextureManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+CustomRenderTextureManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::CustomRenderTextureManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "CustomRenderTextureManager";
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
#[cfg(feature = "UnityEngine+CustomRenderTextureManager")]
impl std::ops::Deref for crate::UnityEngine::CustomRenderTextureManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+CustomRenderTextureManager")]
impl std::ops::DerefMut for crate::UnityEngine::CustomRenderTextureManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+CustomRenderTextureManager")]
impl crate::UnityEngine::CustomRenderTextureManager {
    pub fn InvokeOnTextureLoaded_Internal(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::CustomRenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::CustomRenderTextureManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::CustomRenderTexture>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InvokeOnTextureLoaded_Internal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::CustomRenderTextureManager as
                    quest_hook::libil2cpp::Type > ::class(),
                    "InvokeOnTextureLoaded_Internal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnTextureUnloaded_Internal(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::CustomRenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::CustomRenderTextureManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::CustomRenderTexture>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InvokeOnTextureUnloaded_Internal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::CustomRenderTextureManager as
                    quest_hook::libil2cpp::Type > ::class(),
                    "InvokeOnTextureUnloaded_Internal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (source))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+CustomRenderTextureManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::CustomRenderTextureManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
