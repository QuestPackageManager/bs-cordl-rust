#[cfg(feature = "AudioTypeHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioTypeHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "AudioTypeHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::AudioTypeHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AudioTypeHelper";
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
#[cfg(feature = "AudioTypeHelper")]
impl std::ops::Deref for crate::GlobalNamespace::AudioTypeHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioTypeHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::AudioTypeHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioTypeHelper")]
impl crate::GlobalNamespace::AudioTypeHelper {
    pub fn GetAudioTypeFromPath(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::AudioType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::UnityEngine::AudioType,
                1usize,
            >("GetAudioTypeFromPath")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetAudioTypeFromPath", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::AudioType = unsafe {
            method.invoke_unchecked((), (path))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AudioTypeHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AudioTypeHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
