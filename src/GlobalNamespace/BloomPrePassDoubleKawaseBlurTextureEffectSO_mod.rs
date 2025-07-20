#[cfg(feature = "BloomPrePassDoubleKawaseBlurTextureEffectSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassDoubleKawaseBlurTextureEffectSO {
    __cordl_parent: crate::GlobalNamespace::BloomPrePassEffectSO,
    pub _bloom1KernelSize: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
    pub _bloom1Boost: f32,
    pub _bloom2KernelSize: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
    pub _bloom2Boost: f32,
    pub _bloom2Alpha: f32,
    pub _downsample: i32,
    pub _gammaCorrection: bool,
    pub _kawaseBlurRenderer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::KawaseBlurRendererSO,
    >,
}
#[cfg(feature = "BloomPrePassDoubleKawaseBlurTextureEffectSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BloomPrePassDoubleKawaseBlurTextureEffectSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BloomPrePassDoubleKawaseBlurTextureEffectSO";
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
#[cfg(feature = "BloomPrePassDoubleKawaseBlurTextureEffectSO")]
impl std::ops::Deref
for crate::GlobalNamespace::BloomPrePassDoubleKawaseBlurTextureEffectSO {
    type Target = crate::GlobalNamespace::BloomPrePassEffectSO;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassDoubleKawaseBlurTextureEffectSO")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BloomPrePassDoubleKawaseBlurTextureEffectSO {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassDoubleKawaseBlurTextureEffectSO")]
impl crate::GlobalNamespace::BloomPrePassDoubleKawaseBlurTextureEffectSO {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Render(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Render")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Render",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (src, dest))?
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
#[cfg(feature = "BloomPrePassDoubleKawaseBlurTextureEffectSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassDoubleKawaseBlurTextureEffectSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
