#[cfg(feature = "cordl_class_ZenjectExtensions")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct ZenjectExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_ZenjectExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::ZenjectExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ZenjectExtensions";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "ZenjectExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::ZenjectExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ZenjectExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::ZenjectExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ZenjectExtensions")]
impl crate::GlobalNamespace::ZenjectExtensions {
    pub fn InstantiatePrefabForComponentAt<T>(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                    ), T, 5usize>("InstantiatePrefabForComponentAt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InstantiatePrefabForComponentAt",
                            5usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            cordl_method_info
                .invoke_unchecked((), (container, prefab, position, rotation, parent))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_ZenjectExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ZenjectExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
