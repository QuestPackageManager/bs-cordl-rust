#[cfg(feature = "BloomPrePassRendererSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassRendererSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _bloomFog: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomFogSO>,
    pub _preallocationData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BloomPrePassRendererSO_PreallocationData,
            >,
        >,
    >,
    pub _lightsRenderingData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomPrePassLightTypeSO>,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BloomPrePassRendererSO_LightsRenderingData,
            >,
        >,
    >,
    pub _commandBuffer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::CommandBuffer,
    >,
    pub _initialized: bool,
    pub _blackTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    pub _lowestResBloomTexture: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::RenderTexture,
    >,
}
#[cfg(feature = "BloomPrePassRendererSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BloomPrePassRendererSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BloomPrePassRendererSO";
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
#[cfg(feature = "BloomPrePassRendererSO")]
impl std::ops::Deref for crate::GlobalNamespace::BloomPrePassRendererSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassRendererSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::BloomPrePassRendererSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassRendererSO")]
impl crate::GlobalNamespace::BloomPrePassRendererSO {
    #[cfg(feature = "BloomPrePassRendererSO+LightsRenderingData")]
    pub type LightsRenderingData = crate::GlobalNamespace::BloomPrePassRendererSO_LightsRenderingData;
    #[cfg(feature = "BloomPrePassRendererSO+PreallocationData")]
    pub type PreallocationData = crate::GlobalNamespace::BloomPrePassRendererSO_PreallocationData;
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Cleanup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Cleanup", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateBloomPrePassRenderTextureIfNeeded(
        &mut self,
        renderTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        bloomPrePassParams: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBloomPrePassParams,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IBloomPrePassParams,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                2usize,
            >("CreateBloomPrePassRenderTextureIfNeeded")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateBloomPrePassRenderTextureIfNeeded", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture> = unsafe {
            method.invoke_unchecked(self, (renderTexture, bloomPrePassParams))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisableBloomFog(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("DisableBloomFog")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DisableBloomFog", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableBloomFog(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("EnableBloomFog")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EnableBloomFog", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraParams(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        projectionMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        viewMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        stereoCameraEyeOffset: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("GetCameraParams")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCameraParams", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (camera, projectionMatrix, viewMatrix, stereoCameraEyeOffset),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Init", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn MatrixLerp(
        &mut self,
        from: crate::UnityEngine::Matrix4x4,
        to: crate::UnityEngine::Matrix4x4,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Matrix4x4, crate::UnityEngine::Matrix4x4, f32),
                crate::UnityEngine::Matrix4x4,
                3usize,
            >("MatrixLerp")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MatrixLerp", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            method.invoke_unchecked(self, (from, to, t))
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
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDisable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnDisable", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnEnable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnEnable", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareLightsMeshRendering(
        &mut self,
        lightType: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomPrePassLightTypeSO,
        >,
        data: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomPrePassRendererSO_LightsRenderingData,
        >,
        numberOfLights: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::BloomPrePassLightTypeSO,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::BloomPrePassRendererSO_LightsRenderingData,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("PrepareLightsMeshRendering")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PrepareLightsMeshRendering", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (lightType, data, numberOfLights))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderAllLights(
        &mut self,
        viewMatrix: crate::UnityEngine::Matrix4x4,
        projectionMatrix: crate::UnityEngine::Matrix4x4,
        linesWidth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Matrix4x4, crate::UnityEngine::Matrix4x4, f32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("RenderAllLights")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RenderAllLights", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewMatrix, projectionMatrix, linesWidth))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderAndSetData(
        &mut self,
        cameraPos: crate::UnityEngine::Vector3,
        projectionMatrix: crate::UnityEngine::Matrix4x4,
        viewMatrix: crate::UnityEngine::Matrix4x4,
        stereoCameraEyeOffset: f32,
        bloomPrePassParams: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBloomPrePassParams,
        >,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        textureToScreenRatio: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Vector2,
        >,
        toneMapping: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::ToneMapping>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Matrix4x4,
                    crate::UnityEngine::Matrix4x4,
                    f32,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IBloomPrePassParams,
                    >,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                    quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::ToneMapping>,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >("RenderAndSetData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RenderAndSetData", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        cameraPos,
                        projectionMatrix,
                        viewMatrix,
                        stereoCameraEyeOffset,
                        bloomPrePassParams,
                        dest,
                        textureToScreenRatio,
                        toneMapping,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetCustomStereoCameraEyeOffset(
        &mut self,
        stereoCameraEyeOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetCustomStereoCameraEyeOffset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetCustomStereoCameraEyeOffset", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (stereoCameraEyeOffset))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetDataToShaders(
        stereoCameraEyeOffset: f32,
        textureToScreenRatio: crate::UnityEngine::Vector2,
        bloomFogTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        toneMapping: crate::GlobalNamespace::ToneMapping,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    f32,
                    crate::UnityEngine::Vector2,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                    crate::GlobalNamespace::ToneMapping,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetDataToShaders")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetDataToShaders", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        stereoCameraEyeOffset,
                        textureToScreenRatio,
                        bloomFogTexture,
                        toneMapping,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateBloomFogParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("UpdateBloomFogParams")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateBloomFogParams", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BloomPrePassRendererSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassRendererSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BloomPrePassRendererSO+LightsRenderingData")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassRendererSO_LightsRenderingData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    pub lightQuads: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::BloomPrePassLight_QuadData,
        >,
    >,
    pub subMeshDescriptor: crate::UnityEngine::Rendering::SubMeshDescriptor,
}
#[cfg(feature = "BloomPrePassRendererSO+LightsRenderingData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BloomPrePassRendererSO_LightsRenderingData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BloomPrePassRendererSO/LightsRenderingData";
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
#[cfg(feature = "BloomPrePassRendererSO+LightsRenderingData")]
impl std::ops::Deref
for crate::GlobalNamespace::BloomPrePassRendererSO_LightsRenderingData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassRendererSO+LightsRenderingData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BloomPrePassRendererSO_LightsRenderingData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassRendererSO+LightsRenderingData")]
impl crate::GlobalNamespace::BloomPrePassRendererSO_LightsRenderingData {
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BloomPrePassRendererSO+LightsRenderingData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassRendererSO_LightsRenderingData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BloomPrePassRendererSO+PreallocationData")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassRendererSO_PreallocationData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub lightType: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassLightTypeSO,
    >,
    pub preallocateCount: i32,
}
#[cfg(feature = "BloomPrePassRendererSO+PreallocationData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BloomPrePassRendererSO_PreallocationData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BloomPrePassRendererSO/PreallocationData";
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
#[cfg(feature = "BloomPrePassRendererSO+PreallocationData")]
impl std::ops::Deref
for crate::GlobalNamespace::BloomPrePassRendererSO_PreallocationData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassRendererSO+PreallocationData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BloomPrePassRendererSO_PreallocationData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassRendererSO+PreallocationData")]
impl crate::GlobalNamespace::BloomPrePassRendererSO_PreallocationData {
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BloomPrePassRendererSO+PreallocationData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassRendererSO_PreallocationData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
