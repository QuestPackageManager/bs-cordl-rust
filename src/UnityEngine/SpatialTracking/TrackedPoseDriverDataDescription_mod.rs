#[cfg(
    feature = "cordl_class_UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TrackedPoseDriverDataDescription {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.SpatialTracking";
    const CLASS_NAME: &'static str = "TrackedPoseDriverDataDescription";
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
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription")]
impl std::ops::Deref
for crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription")]
impl std::ops::DerefMut
for crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription")]
impl crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription {
    #[cfg(
        feature = "UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription+PoseData"
    )]
    pub type PoseData = crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription_PoseData;
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
}
#[cfg(
    feature = "cordl_class_UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription+PoseData"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrackedPoseDriverDataDescription_PoseData {
    pub PoseNames: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub Poses: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackedPose,
        >,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription+PoseData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription_PoseData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.SpatialTracking";
    const CLASS_NAME: &'static str = "TrackedPoseDriverDataDescription/PoseData";
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
    feature = "cordl_class_UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription+PoseData"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription_PoseData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription+PoseData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription_PoseData {
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
    feature = "cordl_class_UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription+PoseData"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription_PoseData {
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
    feature = "cordl_class_UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription+PoseData"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription_PoseData {
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
#[cfg(
    feature = "cordl_class_UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription+PoseData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription_PoseData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription+PoseData")]
impl crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription_PoseData {}
