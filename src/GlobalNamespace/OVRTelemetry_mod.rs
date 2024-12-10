#[cfg(feature = "OVRTelemetry")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRTelemetry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTelemetry => ""
    ."OVRTelemetry"
);
#[cfg(feature = "OVRTelemetry")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetry")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetry")]
impl crate::GlobalNamespace::OVRTelemetry {
    #[cfg(feature = "OVRTelemetry+MarkerPoint")]
    pub type MarkerPoint = crate::GlobalNamespace::OVRTelemetry_MarkerPoint;
    #[cfg(feature = "OVRTelemetry+NullTelemetryClient")]
    pub type NullTelemetryClient = crate::GlobalNamespace::OVRTelemetry_NullTelemetryClient;
    #[cfg(feature = "OVRTelemetry+QPLTelemetryClient")]
    pub type QPLTelemetryClient = crate::GlobalNamespace::OVRTelemetry_QPLTelemetryClient;
    #[cfg(feature = "OVRTelemetry+TelemetryClient")]
    pub type TelemetryClient = crate::GlobalNamespace::OVRTelemetry_TelemetryClient;
}
#[cfg(feature = "OVRTelemetry")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRTelemetry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRTelemetry+MarkerPoint")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRTelemetry_MarkerPoint {
    pub _NameHandle_k__BackingField: i32,
}
#[cfg(feature = "OVRTelemetry+MarkerPoint")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTelemetry_MarkerPoint => ""
    ."OVRTelemetry/MarkerPoint"
);
#[cfg(feature = "OVRTelemetry+MarkerPoint")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRTelemetry_MarkerPoint {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRTelemetry+MarkerPoint")]
impl crate::GlobalNamespace::OVRTelemetry_MarkerPoint {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (name),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NameHandle(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_NameHandle",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRTelemetry+MarkerPoint")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::OVRTelemetry_MarkerPoint {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "OVRTelemetry+MarkerPoint")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::OVRTelemetry_MarkerPoint {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "OVRTelemetry+NullTelemetryClient")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetry_NullTelemetryClient {
    __cordl_parent: crate::GlobalNamespace::OVRTelemetry_TelemetryClient,
}
#[cfg(feature = "OVRTelemetry+NullTelemetryClient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRTelemetry_NullTelemetryClient => ""
    ."OVRTelemetry/NullTelemetryClient"
);
#[cfg(feature = "OVRTelemetry+NullTelemetryClient")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetry_NullTelemetryClient {
    type Target = crate::GlobalNamespace::OVRTelemetry_TelemetryClient;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetry+NullTelemetryClient")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetry_NullTelemetryClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetry+NullTelemetryClient")]
impl crate::GlobalNamespace::OVRTelemetry_NullTelemetryClient {
    pub fn CreateMarkerHandle(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nameHandle: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CreateMarkerHandle", (name, nameHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyMarkerHandle(
        &mut self,
        nameHandle: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("DestroyMarkerHandle", (nameHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkerAnnotation(
        &mut self,
        markerId: i32,
        annotationKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        annotationValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        instanceKey: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "MarkerAnnotation",
                (markerId, annotationKey, annotationValue, instanceKey),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkerEnd(
        &mut self,
        markerId: i32,
        resultTypeId: crate::GlobalNamespace::Qpl_OVRPlugin_ResultType,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkerEnd", (markerId, resultTypeId, instanceKey, timestampMs))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkerPointCached(
        &mut self,
        markerId: i32,
        nameHandle: i32,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "MarkerPointCached",
                (markerId, nameHandle, instanceKey, timestampMs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkerStart(
        &mut self,
        markerId: i32,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkerStart", (markerId, instanceKey, timestampMs))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRTelemetry+NullTelemetryClient")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRTelemetry_NullTelemetryClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRTelemetry+QPLTelemetryClient")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetry_QPLTelemetryClient {
    __cordl_parent: crate::GlobalNamespace::OVRTelemetry_TelemetryClient,
}
#[cfg(feature = "OVRTelemetry+QPLTelemetryClient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTelemetry_QPLTelemetryClient
    => ""."OVRTelemetry/QPLTelemetryClient"
);
#[cfg(feature = "OVRTelemetry+QPLTelemetryClient")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetry_QPLTelemetryClient {
    type Target = crate::GlobalNamespace::OVRTelemetry_TelemetryClient;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetry+QPLTelemetryClient")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetry_QPLTelemetryClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetry+QPLTelemetryClient")]
impl crate::GlobalNamespace::OVRTelemetry_QPLTelemetryClient {
    pub fn CreateMarkerHandle(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nameHandle: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CreateMarkerHandle", (name, nameHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyMarkerHandle(
        &mut self,
        nameHandle: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("DestroyMarkerHandle", (nameHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkerAnnotation(
        &mut self,
        markerId: i32,
        annotationKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        annotationValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        instanceKey: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "MarkerAnnotation",
                (markerId, annotationKey, annotationValue, instanceKey),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkerEnd(
        &mut self,
        markerId: i32,
        resultTypeId: crate::GlobalNamespace::Qpl_OVRPlugin_ResultType,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkerEnd", (markerId, resultTypeId, instanceKey, timestampMs))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkerPointCached(
        &mut self,
        markerId: i32,
        nameHandle: i32,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "MarkerPointCached",
                (markerId, nameHandle, instanceKey, timestampMs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkerStart(
        &mut self,
        markerId: i32,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkerStart", (markerId, instanceKey, timestampMs))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRTelemetry+QPLTelemetryClient")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRTelemetry_QPLTelemetryClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRTelemetry+TelemetryClient")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTelemetry_TelemetryClient {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRTelemetry+TelemetryClient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTelemetry_TelemetryClient =>
    ""."OVRTelemetry/TelemetryClient"
);
#[cfg(feature = "OVRTelemetry+TelemetryClient")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTelemetry_TelemetryClient {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetry+TelemetryClient")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTelemetry_TelemetryClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTelemetry+TelemetryClient")]
impl crate::GlobalNamespace::OVRTelemetry_TelemetryClient {
    pub fn CreateMarkerHandle(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nameHandle: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CreateMarkerHandle", (name, nameHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyMarkerHandle(
        &mut self,
        nameHandle: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("DestroyMarkerHandle", (nameHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkerAnnotation(
        &mut self,
        markerId: i32,
        annotationKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        annotationValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        instanceKey: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "MarkerAnnotation",
                (markerId, annotationKey, annotationValue, instanceKey),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkerEnd(
        &mut self,
        markerId: i32,
        resultTypeId: crate::GlobalNamespace::Qpl_OVRPlugin_ResultType,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkerEnd", (markerId, resultTypeId, instanceKey, timestampMs))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkerPointCached(
        &mut self,
        markerId: i32,
        nameHandle: i32,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "MarkerPointCached",
                (markerId, nameHandle, instanceKey, timestampMs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkerStart(
        &mut self,
        markerId: i32,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkerStart", (markerId, instanceKey, timestampMs))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRTelemetry+TelemetryClient")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRTelemetry_TelemetryClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
