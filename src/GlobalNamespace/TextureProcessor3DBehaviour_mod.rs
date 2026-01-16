#[cfg(feature = "cordl_class_TextureProcessor3DBehaviour")]
#[repr(C)]
#[derive(Debug)]
pub struct TextureProcessor3DBehaviour {
    __cordl_parent: crate::UnityEngine::Playables::PlayableBehaviour,
    pub computeKernelA: f32,
    pub computeKernelB: f32,
    pub computeKernelC: f32,
    pub computeKernelD: f32,
    pub inputTextureIndexA: f32,
    pub inputTextureIndexB: f32,
    pub inputTextureIndexC: f32,
    pub inputTextureIndexD: f32,
    pub speedA: f32,
    pub speedB: f32,
    pub speedC: f32,
    pub speedD: f32,
    pub spatialScaleA: f32,
    pub spatialScaleB: f32,
    pub spatialScaleC: f32,
    pub spatialScaleD: f32,
    pub phaseA: f32,
    pub phaseB: f32,
    pub phaseC: f32,
    pub phaseD: f32,
    pub param2A: f32,
    pub param2B: f32,
    pub param2C: f32,
    pub param2D: f32,
    pub param1A: f32,
    pub param1B: f32,
    pub param1C: f32,
    pub param1D: f32,
    pub _initialized: bool,
    pub _originalComputeKernelA: f32,
    pub _originalComputeKernelB: f32,
    pub _originalComputeKernelC: f32,
    pub _originalComputeKernelD: f32,
    pub _originalInputTextureIndexA: f32,
    pub _originalInputTextureIndexB: f32,
    pub _originalInputTextureIndexC: f32,
    pub _originalInputTextureIndexD: f32,
    pub _originalSpeedA: f32,
    pub _originalSpeedB: f32,
    pub _originalSpeedC: f32,
    pub _originalSpeedD: f32,
    pub _originalSpatialScaleA: f32,
    pub _originalSpatialScaleB: f32,
    pub _originalSpatialScaleC: f32,
    pub _originalSpatialScaleD: f32,
    pub _phaseA: f32,
    pub _phaseB: f32,
    pub _phaseC: f32,
    pub _phaseD: f32,
    pub _param2A: f32,
    pub _param2B: f32,
    pub _param2C: f32,
    pub _param2D: f32,
    pub _param1A: f32,
    pub _param1B: f32,
    pub _param1C: f32,
    pub _param1D: f32,
    pub _textureProcessor: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TextureProcessor3D,
    >,
}
#[cfg(feature = "cordl_class_TextureProcessor3DBehaviour")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TextureProcessor3DBehaviour {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TextureProcessor3DBehaviour";
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
#[cfg(feature = "TextureProcessor3DBehaviour")]
impl std::ops::Deref for crate::GlobalNamespace::TextureProcessor3DBehaviour {
    type Target = crate::UnityEngine::Playables::PlayableBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TextureProcessor3DBehaviour")]
impl std::ops::DerefMut for crate::GlobalNamespace::TextureProcessor3DBehaviour {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TextureProcessor3DBehaviour")]
impl crate::GlobalNamespace::TextureProcessor3DBehaviour {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnPlayableDestroy(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Playables::Playable),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnPlayableDestroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnPlayableDestroy", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (playable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessFrame(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
        playerData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Playables::Playable,
                            crate::UnityEngine::Playables::FrameData,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ProcessFrame")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessFrame", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (playable, info, playerData))?
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
#[cfg(feature = "cordl_class_TextureProcessor3DBehaviour")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TextureProcessor3DBehaviour {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
