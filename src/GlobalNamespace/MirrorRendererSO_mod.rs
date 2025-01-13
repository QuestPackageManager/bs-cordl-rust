#[cfg(feature = "MirrorRendererSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MirrorRendererSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _reflectLayers: crate::UnityEngine::LayerMask,
    pub _stereoTextureWidth: i32,
    pub _stereoTextureHeight: i32,
    pub _monoTextureWidth: i32,
    pub _monoTextureHeight: i32,
    pub _maxAntiAliasing: i32,
    pub _disableDepthTexture: bool,
    pub _enableBloomPrePass: bool,
    pub _bloomPrePassRenderer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassRendererSO,
    >,
    pub _bloomPrePassEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassEffectSO,
    >,
    pub _clearDepthShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub _bloomPrePassRenderTexture: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::RenderTexture,
    >,
    pub _mirrorCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub _antialiasing: i32,
    pub _renderTextures: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::GlobalNamespace::MirrorRendererSO_CameraTransformData,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        >,
    >,
    pub kLeftRect: crate::UnityEngine::Rect,
    pub kRightRect: crate::UnityEngine::Rect,
    pub kFullRect: crate::UnityEngine::Rect,
}
#[cfg(feature = "MirrorRendererSO")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::MirrorRendererSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MirrorRendererSO";
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
#[cfg(feature = "MirrorRendererSO")]
impl std::ops::Deref for crate::GlobalNamespace::MirrorRendererSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MirrorRendererSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::MirrorRendererSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MirrorRendererSO")]
impl crate::GlobalNamespace::MirrorRendererSO {
    pub const kWaterLayer: i32 = 4i32;
    #[cfg(feature = "MirrorRendererSO+CameraTransformData")]
    pub type CameraTransformData = crate::GlobalNamespace::MirrorRendererSO_CameraTransformData;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateReflectionMatrix(
        plane: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateReflectionMatrix", (plane))?;
        Ok(__cordl_ret.into())
    }
    pub fn CameraSpacePlane(
        worldToCameraMatrix: crate::UnityEngine::Matrix4x4,
        pos: crate::UnityEngine::Vector3,
        normal: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CameraSpacePlane", (worldToCameraMatrix, pos, normal))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateOrUpdateMirrorCamera(
        &mut self,
        currentCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        renderTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateOrUpdateMirrorCamera", (currentCamera, renderTexture))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        reflectLayers: crate::UnityEngine::LayerMask,
        stereoTextureWidth: i32,
        stereoTextureHeight: i32,
        monoTextureWidth: i32,
        monoTextureHeight: i32,
        maxAntiAliasing: i32,
        enableBloomPrePass: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    reflectLayers,
                    stereoTextureWidth,
                    stereoTextureHeight,
                    monoTextureWidth,
                    monoTextureHeight,
                    maxAntiAliasing,
                    enableBloomPrePass,
                ),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnValidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValidate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Plane(
        pos: crate::UnityEngine::Vector3,
        normal: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Plane", (pos, normal))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareForNextFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareForNextFrame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderMirror(
        &mut self,
        camPosition: crate::UnityEngine::Vector3,
        camRotation: crate::UnityEngine::Quaternion,
        camProjectionMatrix: crate::UnityEngine::Matrix4x4,
        screenRect: crate::UnityEngine::Rect,
        reclectionPlanePos: crate::UnityEngine::Vector3,
        reflectionPlaneNormal: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RenderMirror",
                (
                    camPosition,
                    camRotation,
                    camProjectionMatrix,
                    screenRect,
                    reclectionPlanePos,
                    reflectionPlaneNormal,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderMirrorTexture(
        &mut self,
        reflectionPlanePos: crate::UnityEngine::Vector3,
        reflectionPlaneNormal: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = __cordl_object
            .invoke("RenderMirrorTexture", (reflectionPlanePos, reflectionPlaneNormal))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateParams", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MirrorRendererSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MirrorRendererSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MirrorRendererSO+CameraTransformData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MirrorRendererSO_CameraTransformData {
    pub position: crate::UnityEngine::Vector3,
    pub rotation: crate::UnityEngine::Quaternion,
    pub fov: f32,
    pub stereoEnabled: bool,
}
#[cfg(feature = "MirrorRendererSO+CameraTransformData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MirrorRendererSO_CameraTransformData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MirrorRendererSO/CameraTransformData";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "MirrorRendererSO+CameraTransformData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::MirrorRendererSO_CameraTransformData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "MirrorRendererSO+CameraTransformData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::MirrorRendererSO_CameraTransformData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "MirrorRendererSO+CameraTransformData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::MirrorRendererSO_CameraTransformData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "MirrorRendererSO+CameraTransformData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::MirrorRendererSO_CameraTransformData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "MirrorRendererSO+CameraTransformData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::MirrorRendererSO_CameraTransformData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "MirrorRendererSO+CameraTransformData")]
impl crate::GlobalNamespace::MirrorRendererSO_CameraTransformData {
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_MirrorRendererSO_CameraTransformData0(
        &mut self,
        other: crate::GlobalNamespace::MirrorRendererSO_CameraTransformData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::GlobalNamespace::MirrorRendererSO_CameraTransformData,
        right: crate::GlobalNamespace::MirrorRendererSO_CameraTransformData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        left: crate::GlobalNamespace::MirrorRendererSO_CameraTransformData,
        right: crate::GlobalNamespace::MirrorRendererSO_CameraTransformData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MirrorRendererSO+CameraTransformData")]
impl AsRef<
    crate::System::IEquatable_1<
        crate::GlobalNamespace::MirrorRendererSO_CameraTransformData,
    >,
> for crate::GlobalNamespace::MirrorRendererSO_CameraTransformData {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::GlobalNamespace::MirrorRendererSO_CameraTransformData,
    > {
        todo!()
    }
}
#[cfg(feature = "MirrorRendererSO+CameraTransformData")]
impl AsMut<
    crate::System::IEquatable_1<
        crate::GlobalNamespace::MirrorRendererSO_CameraTransformData,
    >,
> for crate::GlobalNamespace::MirrorRendererSO_CameraTransformData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::GlobalNamespace::MirrorRendererSO_CameraTransformData,
    > {
        todo!()
    }
}
