#[cfg(feature = "TrackLaneRingsRotationEffect+RingRotationEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackLaneRingsRotationEffect_RingRotationEffect {
    __cordl_parent: crate::System::Object,
    pub rotationAngle: f32,
    pub rotationStep: f32,
    pub rotationFlexySpeed: f32,
    pub rotationPropagationSpeed: i32,
    pub progressPos: i32,
}
#[cfg(feature = "TrackLaneRingsRotationEffect+RingRotationEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TrackLaneRingsRotationEffect_RingRotationEffect => ""
    ."TrackLaneRingsRotationEffect/RingRotationEffect"
);
#[cfg(feature = "TrackLaneRingsRotationEffect+RingRotationEffect")]
impl std::ops::Deref
for crate::GlobalNamespace::TrackLaneRingsRotationEffect_RingRotationEffect {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TrackLaneRingsRotationEffect+RingRotationEffect")]
impl std::ops::DerefMut
for crate::GlobalNamespace::TrackLaneRingsRotationEffect_RingRotationEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TrackLaneRingsRotationEffect+RingRotationEffect")]
impl crate::GlobalNamespace::TrackLaneRingsRotationEffect_RingRotationEffect {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TrackLaneRingsRotationEffect+RingRotationEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TrackLaneRingsRotationEffect_RingRotationEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TrackLaneRingsRotationEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackLaneRingsRotationEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _trackLaneRingsManager: *mut TrackLaneRingsManager,
    pub _startupRotationAngle: f32,
    pub _startupRotationStep: f32,
    pub _startupRotationPropagationSpeed: i32,
    pub _startupRotationFlexySpeed: f32,
    pub _activeRingRotationEffects: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::TrackLaneRingsRotationEffect_RingRotationEffect,
    >,
    pub _ringRotationEffectsPool: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::TrackLaneRingsRotationEffect_RingRotationEffect,
    >,
    pub ringRotationEffectsToDelete: *mut crate::System::Collections::Generic::List_1<
        i32,
    >,
}
#[cfg(feature = "TrackLaneRingsRotationEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for TrackLaneRingsRotationEffect => ""
    ."TrackLaneRingsRotationEffect"
);
#[cfg(feature = "TrackLaneRingsRotationEffect")]
impl std::ops::Deref for TrackLaneRingsRotationEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TrackLaneRingsRotationEffect")]
impl std::ops::DerefMut for TrackLaneRingsRotationEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TrackLaneRingsRotationEffect")]
impl TrackLaneRingsRotationEffect {
    #[cfg(feature = "TrackLaneRingsRotationEffect+RingRotationEffect")]
    pub type RingRotationEffect = crate::GlobalNamespace::TrackLaneRingsRotationEffect_RingRotationEffect;
    pub fn AddRingRotationEffect(
        &mut self,
        angle: f32,
        step: f32,
        propagationSpeed: i32,
        flexySpeed: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddRingRotationEffect",
                (angle, step, propagationSpeed, flexySpeed),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn FixedUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixedUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetFirstRingDestinationRotationAngle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetFirstRingDestinationRotationAngle", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetFirstRingRotationAngle(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetFirstRingRotationAngle", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RecycleRingRotationEffect(
        &mut self,
        ringRotationEffect: *mut crate::GlobalNamespace::TrackLaneRingsRotationEffect_RingRotationEffect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecycleRingRotationEffect", (ringRotationEffect))?;
        Ok(__cordl_ret)
    }
    pub fn SpawnRingRotationEffect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::TrackLaneRingsRotationEffect_RingRotationEffect,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::TrackLaneRingsRotationEffect_RingRotationEffect = __cordl_object
            .invoke("SpawnRingRotationEffect", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TrackLaneRingsRotationEffect")]
impl quest_hook::libil2cpp::ObjectType for TrackLaneRingsRotationEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
