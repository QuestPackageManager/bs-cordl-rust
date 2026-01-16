#[cfg(feature = "cordl_class_UnityEngine+VFX+VFXSpawnerCallbacks")]
#[repr(C)]
#[derive(Debug)]
pub struct VFXSpawnerCallbacks {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
}
#[cfg(feature = "cordl_class_UnityEngine+VFX+VFXSpawnerCallbacks")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::VFX::VFXSpawnerCallbacks {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.VFX";
    const CLASS_NAME: &'static str = "VFXSpawnerCallbacks";
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
#[cfg(feature = "UnityEngine+VFX+VFXSpawnerCallbacks")]
impl std::ops::Deref for crate::UnityEngine::VFX::VFXSpawnerCallbacks {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+VFX+VFXSpawnerCallbacks")]
impl std::ops::DerefMut for crate::UnityEngine::VFX::VFXSpawnerCallbacks {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+VFX+VFXSpawnerCallbacks")]
impl crate::UnityEngine::VFX::VFXSpawnerCallbacks {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnPlay(
        &mut self,
        state: quest_hook::libil2cpp::Gc<crate::UnityEngine::VFX::VFXSpawnerState>,
        vfxValues: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::VFX::VFXExpressionValues,
        >,
        vfxComponent: quest_hook::libil2cpp::Gc<crate::UnityEngine::VFX::VisualEffect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::VFX::VFXSpawnerState,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::VFX::VFXExpressionValues,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::VFX::VisualEffect,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("OnPlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "OnPlay",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (state, vfxValues, vfxComponent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnStop(
        &mut self,
        state: quest_hook::libil2cpp::Gc<crate::UnityEngine::VFX::VFXSpawnerState>,
        vfxValues: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::VFX::VFXExpressionValues,
        >,
        vfxComponent: quest_hook::libil2cpp::Gc<crate::UnityEngine::VFX::VisualEffect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::VFX::VFXSpawnerState,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::VFX::VFXExpressionValues,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::VFX::VisualEffect,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("OnStop")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "OnStop",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (state, vfxValues, vfxComponent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnUpdate(
        &mut self,
        state: quest_hook::libil2cpp::Gc<crate::UnityEngine::VFX::VFXSpawnerState>,
        vfxValues: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::VFX::VFXExpressionValues,
        >,
        vfxComponent: quest_hook::libil2cpp::Gc<crate::UnityEngine::VFX::VisualEffect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::VFX::VFXSpawnerState,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::VFX::VFXExpressionValues,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::VFX::VisualEffect,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("OnUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnUpdate", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (state, vfxValues, vfxComponent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+VFX+VFXSpawnerCallbacks")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::VFX::VFXSpawnerCallbacks {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
