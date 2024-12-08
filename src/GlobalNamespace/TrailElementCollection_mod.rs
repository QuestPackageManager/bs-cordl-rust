#[cfg(feature = "TrailElementCollection+InterpolationState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TrailElementCollection_InterpolationState {
    pub segmentIndex: i32,
    pub segmentLerp: f32,
}
#[cfg(feature = "TrailElementCollection+InterpolationState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TrailElementCollection_InterpolationState => ""
    ."TrailElementCollection/InterpolationState"
);
#[cfg(feature = "TrailElementCollection+InterpolationState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::TrailElementCollection_InterpolationState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TrailElementCollection+InterpolationState")]
impl crate::GlobalNamespace::TrailElementCollection_InterpolationState {}
#[cfg(feature = "TrailElementCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct TrailElementCollection {
    __cordl_parent: crate::System::Object,
    pub _capacity: i32,
    pub _snapshots: *mut quest_hook::libil2cpp::Il2CppArray<*mut TrailElement>,
    pub _headIndex: i32,
    pub _totalDistance: f32,
}
#[cfg(feature = "TrailElementCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for TrailElementCollection => ""."TrailElementCollection"
);
#[cfg(feature = "TrailElementCollection")]
impl std::ops::Deref for TrailElementCollection {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TrailElementCollection")]
impl std::ops::DerefMut for TrailElementCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TrailElementCollection")]
impl TrailElementCollection {
    #[cfg(feature = "TrailElementCollection+InterpolationState")]
    pub type InterpolationState = crate::GlobalNamespace::TrailElementCollection_InterpolationState;
    pub fn GetElement(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut TrailElement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut TrailElement = __cordl_object
            .invoke("GetElement", (index))?;
        Ok(__cordl_ret)
    }
    pub fn InitSnapshots(
        &mut self,
        defaultStartPosition: crate::UnityEngine::Vector3,
        defaultEndPosition: crate::UnityEngine::Vector3,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InitSnapshots",
                (defaultStartPosition, defaultEndPosition, _cordl_time),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Interpolate(
        &mut self,
        t: f32,
        lerpState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::TrailElementCollection_InterpolationState,
        >,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        normal: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        _cordl_time: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Interpolate", (t, lerpState, position, normal, _cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn MoveTailToHead(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveTailToHead", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        capacity: i32,
        defaultStartPosition: crate::UnityEngine::Vector3,
        defaultEndPosition: crate::UnityEngine::Vector3,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (capacity, defaultStartPosition, defaultEndPosition, _cordl_time),
            )?;
        Ok(__cordl_object)
    }
    pub fn SetHeadData(
        &mut self,
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHeadData", (start, end, _cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateDistances(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDistances", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateLerpState(
        &mut self,
        t: f32,
        interpolationState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::TrailElementCollection_InterpolationState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateLerpState", (t, interpolationState))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        capacity: i32,
        defaultStartPosition: crate::UnityEngine::Vector3,
        defaultEndPosition: crate::UnityEngine::Vector3,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (capacity, defaultStartPosition, defaultEndPosition, _cordl_time),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TrailElementCollection")]
impl quest_hook::libil2cpp::ObjectType for TrailElementCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}