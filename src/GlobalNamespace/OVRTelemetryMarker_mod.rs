#[cfg(feature = "OVRTelemetryMarker")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRTelemetryMarker {
    pub _State_k__BackingField: crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState,
    pub _MarkerId_k__BackingField: i32,
    pub _InstanceKey_k__BackingField: i32,
    pub _client: *mut crate::GlobalNamespace::OVRTelemetry_TelemetryClient,
}
#[cfg(feature = "OVRTelemetryMarker")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTelemetryMarker => ""
    ."OVRTelemetryMarker"
);
#[cfg(feature = "OVRTelemetryMarker")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRTelemetryMarker {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRTelemetryMarker")]
impl crate::GlobalNamespace::OVRTelemetryMarker {
    #[cfg(feature = "OVRTelemetryMarker+OVRTelemetryMarkerState")]
    pub type OVRTelemetryMarkerState = crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState;
    pub fn AddAnnotation(
        &mut self,
        annotationKey: *mut crate::System::String,
        annotationValue: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTelemetryMarker> {
        let __cordl_ret: crate::GlobalNamespace::OVRTelemetryMarker = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddAnnotation",
            (annotationKey, annotationValue),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AddPoint(
        &mut self,
        point: crate::GlobalNamespace::OVRTelemetry_MarkerPoint,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTelemetryMarker> {
        let __cordl_ret: crate::GlobalNamespace::OVRTelemetryMarker = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddPoint",
            (point),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Send(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTelemetryMarker> {
        let __cordl_ret: crate::GlobalNamespace::OVRTelemetryMarker = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Send",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SendIf(
        &mut self,
        condition: bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTelemetryMarker> {
        let __cordl_ret: crate::GlobalNamespace::OVRTelemetryMarker = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SendIf",
            (condition),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetResult(
        &mut self,
        result: crate::GlobalNamespace::Qpl_ResultType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTelemetryMarker> {
        let __cordl_ret: crate::GlobalNamespace::OVRTelemetryMarker = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetResult",
            (result),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_OVRTelemetry_TelemetryClient_i32_i64_1(
        &mut self,
        client: *mut crate::GlobalNamespace::OVRTelemetry_TelemetryClient,
        markerId: i32,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (client, markerId, instanceKey, timestampMs),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i64_0(
        &mut self,
        markerId: i32,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (markerId, instanceKey, timestampMs),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_InstanceKey(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_InstanceKey",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_MarkerId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_MarkerId",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Result(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::Qpl_ResultType> {
        let __cordl_ret: crate::GlobalNamespace::Qpl_ResultType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Result",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Sent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Sent",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_State(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_State",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_State(
        &mut self,
        value: crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_State",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRTelemetryMarker+OVRTelemetryMarkerState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRTelemetryMarker_OVRTelemetryMarkerState {
    pub _Sent_k__BackingField: bool,
    pub _Result_k__BackingField: crate::GlobalNamespace::Qpl_ResultType,
}
#[cfg(feature = "OVRTelemetryMarker+OVRTelemetryMarkerState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState => ""
    ."OVRTelemetryMarker/OVRTelemetryMarkerState"
);
#[cfg(feature = "OVRTelemetryMarker+OVRTelemetryMarkerState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRTelemetryMarker+OVRTelemetryMarkerState")]
impl crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState {
    pub fn _ctor(
        &mut self,
        sent: bool,
        result: crate::GlobalNamespace::Qpl_ResultType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (sent, result),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Result(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::Qpl_ResultType> {
        let __cordl_ret: crate::GlobalNamespace::Qpl_ResultType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Result",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Sent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Sent",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_Result(
        &mut self,
        value: crate::GlobalNamespace::Qpl_ResultType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Result",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_Sent(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Sent",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
