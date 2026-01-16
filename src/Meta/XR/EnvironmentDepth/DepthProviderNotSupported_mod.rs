#[cfg(feature = "cordl_class_Meta+XR+EnvironmentDepth+DepthProviderNotSupported")]
#[repr(C)]
#[derive(Debug)]
pub struct DepthProviderNotSupported {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+EnvironmentDepth+DepthProviderNotSupported")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Meta::XR::EnvironmentDepth::DepthProviderNotSupported {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.EnvironmentDepth";
    const CLASS_NAME: &'static str = "DepthProviderNotSupported";
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
#[cfg(feature = "Meta+XR+EnvironmentDepth+DepthProviderNotSupported")]
impl std::ops::Deref for crate::Meta::XR::EnvironmentDepth::DepthProviderNotSupported {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+EnvironmentDepth+DepthProviderNotSupported")]
impl std::ops::DerefMut
for crate::Meta::XR::EnvironmentDepth::DepthProviderNotSupported {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+EnvironmentDepth+DepthProviderNotSupported")]
impl crate::Meta::XR::EnvironmentDepth::DepthProviderNotSupported {
    pub fn Meta_XR_EnvironmentDepth_IDepthProvider_SetDepthEnabled(
        &mut self,
        isEnabled: bool,
        removeHands: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Meta.XR.EnvironmentDepth.IDepthProvider.SetDepthEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Meta.XR.EnvironmentDepth.IDepthProvider.SetDepthEnabled",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (isEnabled, removeHands))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Meta_XR_EnvironmentDepth_IDepthProvider_TryGetUpdatedDepthTexture(
        &mut self,
        depthTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        >,
        frameDescriptors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::Meta::XR::EnvironmentDepth::DepthFrameDesc,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::Meta::XR::EnvironmentDepth::DepthFrameDesc,
                                >,
                            >,
                        ),
                        bool,
                        2usize,
                    >(
                        "Meta.XR.EnvironmentDepth.IDepthProvider.TryGetUpdatedDepthTexture",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Meta.XR.EnvironmentDepth.IDepthProvider.TryGetUpdatedDepthTexture",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (depthTexture, frameDescriptors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Meta_XR_EnvironmentDepth_IDepthProvider_get_IsSupported(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >("Meta.XR.EnvironmentDepth.IDepthProvider.get_IsSupported")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Meta.XR.EnvironmentDepth.IDepthProvider.get_IsSupported",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Meta_XR_EnvironmentDepth_IDepthProvider_set_RemoveHands(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Meta.XR.EnvironmentDepth.IDepthProvider.set_RemoveHands")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Meta.XR.EnvironmentDepth.IDepthProvider.set_RemoveHands",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "cordl_class_Meta+XR+EnvironmentDepth+DepthProviderNotSupported")]
impl quest_hook::libil2cpp::ObjectType
for crate::Meta::XR::EnvironmentDepth::DepthProviderNotSupported {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Meta+XR+EnvironmentDepth+DepthProviderNotSupported")]
impl AsRef<crate::Meta::XR::EnvironmentDepth::IDepthProvider>
for crate::Meta::XR::EnvironmentDepth::DepthProviderNotSupported {
    fn as_ref(&self) -> &crate::Meta::XR::EnvironmentDepth::IDepthProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Meta+XR+EnvironmentDepth+DepthProviderNotSupported")]
impl AsMut<crate::Meta::XR::EnvironmentDepth::IDepthProvider>
for crate::Meta::XR::EnvironmentDepth::DepthProviderNotSupported {
    fn as_mut(&mut self) -> &mut crate::Meta::XR::EnvironmentDepth::IDepthProvider {
        unsafe { std::mem::transmute(self) }
    }
}
