#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO")]
#[repr(C)]
#[derive(Debug)]
pub struct EventTrackDefinitionSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _dataTransformationType: crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_DataTransformationType,
    pub _markerType: crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_MarkerType,
    pub _visible: bool,
    pub _needsFiltering: bool,
}
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.TrackDefinitions";
    const CLASS_NAME: &'static str = "EventTrackDefinitionSO";
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
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO")]
impl std::ops::Deref for crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO")]
impl std::ops::DerefMut for crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO")]
impl crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO {
    #[cfg(
        feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+DataTransformationType"
    )]
    pub type DataTransformationType = crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_DataTransformationType;
    #[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+MarkerType")]
    pub type MarkerType = crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_MarkerType;
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
    pub fn get_dataTransformation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_DataTransformationType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_DataTransformationType,
                        0usize,
                    >("get_dataTransformation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_dataTransformation", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_DataTransformationType = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_markerType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_MarkerType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_MarkerType,
                        0usize,
                    >("get_markerType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_markerType", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_MarkerType = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_needsFiltering(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_needsFiltering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_needsFiltering", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_visible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_visible")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_visible", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+DataTransformationType"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventTrackDefinitionSO_DataTransformationType {
    #[default]
    DeltaRotation = 2i32,
    Light = 1i32,
    NoTransformation = 0i32,
    Switch = 3i32,
    TurnOffValueDuration = 4i32,
    ValueDuration = 5i32,
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+DataTransformationType"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_DataTransformationType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.TrackDefinitions";
    const CLASS_NAME: &'static str = "EventTrackDefinitionSO/DataTransformationType";
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
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+DataTransformationType"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_DataTransformationType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+DataTransformationType"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_DataTransformationType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+DataTransformationType"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_DataTransformationType {
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
#[cfg(
    feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+DataTransformationType"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_DataTransformationType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+MarkerType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventTrackDefinitionSO_MarkerType {
    #[default]
    BasicMarker = 0i32,
    DurationMarker = 1i32,
    LightMarker = 2i32,
    TextMarker = 3i32,
    TooltipMarker = 4i32,
}
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+MarkerType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_MarkerType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.TrackDefinitions";
    const CLASS_NAME: &'static str = "EventTrackDefinitionSO/MarkerType";
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
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+MarkerType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_MarkerType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+MarkerType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_MarkerType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+MarkerType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_MarkerType {
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
#[cfg(feature = "BeatSaber+TrackDefinitions+EventTrackDefinitionSO+MarkerType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatSaber::TrackDefinitions::EventTrackDefinitionSO_MarkerType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
