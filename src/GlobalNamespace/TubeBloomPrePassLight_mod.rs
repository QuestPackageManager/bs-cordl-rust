#[cfg(feature = "TubeBloomPrePassLight")]
#[repr(C)]
#[derive(Debug)]
pub struct TubeBloomPrePassLight {
    __cordl_parent: crate::GlobalNamespace::BloomPrePassLight,
    pub _mainEffectPostProcessEnabled: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BoolSO,
    >,
    pub _width: f32,
    pub _overrideChildrenLength: bool,
    pub _length: f32,
    pub _center: f32,
    pub _color: crate::UnityEngine::Color,
    pub _colorAlphaMultiplier: f32,
    pub _bloomFogIntensityMultiplier: f32,
    pub _fakeBloomIntensityMultiplier: f32,
    pub _boostToWhite: f32,
    pub _lightWidthMultiplier: f32,
    pub _addWidthToLength: bool,
    pub _thickenWithDistance: bool,
    pub _thickenCurve: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
    pub _minDistance: f32,
    pub _maxDistance: f32,
    pub _minWidthMultiplier: f32,
    pub _maxWidthMultiplier: f32,
    pub _disableRenderersOnZeroAlpha: bool,
    pub _bakedGlowWidthScale: f32,
    pub _forceUseBakedGlow: bool,
    pub _multiplyLengthByAlpha: bool,
    pub _alphaToLengthCurve: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationCurve,
    >,
    pub _alphaToLengthBloomFogCurve: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationCurve,
    >,
    pub _updateAlways: bool,
    pub _limitAlpha: bool,
    pub _minAlpha: f32,
    pub _maxAlpha: f32,
    pub _overrideChildrenAlpha: bool,
    pub _startAlpha: f32,
    pub _endAlpha: f32,
    pub _overrideChildrenWidth: bool,
    pub _startWidth: f32,
    pub _endWidth: f32,
    pub _parametricBoxController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ParametricBoxController,
    >,
    pub _dynamic3SliceSprite: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::Parametric3SliceSpriteController,
    >,
    pub _initialized: bool,
    pub _enabledRenderers: bool,
    pub _parametricBoxControllerOnceParInitialized: bool,
    pub _bakedGlowOnceParInitialized: bool,
    pub _isParametricBoxControllerValid: bool,
    pub _isDynamic3SliceSpriteValid: bool,
    pub didRefreshEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _useCollision: bool,
    pub _collisionLength: f32,
    pub _multiplyLengthByAlphaBloomFogMultiplier: f32,
    pub _multiplyLengthByAlphaMultiplier: f32,
    pub _isDirty: bool,
    pub _transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
}
#[cfg(feature = "TubeBloomPrePassLight")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TubeBloomPrePassLight => ""
    ."TubeBloomPrePassLight"
);
#[cfg(feature = "TubeBloomPrePassLight")]
impl std::ops::Deref for crate::GlobalNamespace::TubeBloomPrePassLight {
    type Target = crate::GlobalNamespace::BloomPrePassLight;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TubeBloomPrePassLight")]
impl std::ops::DerefMut for crate::GlobalNamespace::TubeBloomPrePassLight {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TubeBloomPrePassLight")]
impl crate::GlobalNamespace::TubeBloomPrePassLight {
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
    pub fn ClipPoints(
        fromPointClipPos: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        toPointClipPos: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        fromPointViewPos: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        toPointViewPos: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        fromPointInside: bool,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ClipPoints",
                (
                    fromPointClipPos,
                    toPointClipPos,
                    fromPointViewPos,
                    toPointViewPos,
                    fromPointInside,
                    t,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DidRegisterLight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidRegisterLight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FillMeshData(
        &mut self,
        lightNum: quest_hook::libil2cpp::ByRefMut<i32>,
        lightQuads: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::BloomPrePassLight_QuadData,
            >,
        >,
        viewMatrix: crate::UnityEngine::Matrix4x4,
        projectionMatrix: crate::UnityEngine::Matrix4x4,
        lineWidth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "FillMeshData",
                (lightNum, lightQuads, viewMatrix, projectionMatrix, lineWidth),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InitIfNeeded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitIfNeeded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn NeedsRefresh(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("NeedsRefresh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDrawGizmos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDrawGizmos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ZeroQuad(
        quad: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::BloomPrePassLight_QuadData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ZeroQuad", (quad))?;
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
    pub fn add_didRefreshEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didRefreshEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bloomFogIntensityMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_bloomFogIntensityMultiplier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_calculatedCollisionLength(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_calculatedCollisionLength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_center(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_center", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_collisionEndAlpha(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_collisionEndAlpha", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_collisionLength(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_collisionLength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colorAlphaMultiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_colorAlphaMultiplier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enabledRenderers(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enabledRenderers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_length(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_length", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightWidthMultiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_lightWidthMultiplier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startAlpha(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_startAlpha", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useCollision(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useCollision", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_width(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_width", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didRefreshEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didRefreshEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bloomFogIntensityMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bloomFogIntensityMultiplier", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_collisionLength(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_collisionLength", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_color(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_color", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_length(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_length", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lightWidthMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lightWidthMultiplier", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startAlpha(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_startAlpha", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useCollision(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useCollision", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_width(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_width", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TubeBloomPrePassLight")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TubeBloomPrePassLight {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
