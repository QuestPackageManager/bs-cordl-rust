#[cfg(feature = "TrailElementCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct TrailElementCollection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _capacity: i32,
    pub _snapshots: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::TrailElement>,
        >,
    >,
    pub _headIndex: i32,
    pub _totalDistance: f32,
}
#[cfg(feature = "TrailElementCollection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TrailElementCollection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TrailElementCollection";
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
#[cfg(feature = "TrailElementCollection")]
impl std::ops::Deref for crate::GlobalNamespace::TrailElementCollection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TrailElementCollection")]
impl std::ops::DerefMut for crate::GlobalNamespace::TrailElementCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TrailElementCollection")]
impl crate::GlobalNamespace::TrailElementCollection {
    #[cfg(feature = "TrailElementCollection+InterpolationState")]
    pub type InterpolationState = crate::GlobalNamespace::TrailElementCollection_InterpolationState;
    pub fn GetElement(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::TrailElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TrailElement,
        > = __cordl_object.invoke("GetElement", (index))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn MoveTailToHead(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveTailToHead", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        capacity: i32,
        defaultStartPosition: crate::UnityEngine::Vector3,
        defaultEndPosition: crate::UnityEngine::Vector3,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (capacity, defaultStartPosition, defaultEndPosition, _cordl_time),
            )?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn UpdateDistances(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDistances", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TrailElementCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TrailElementCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TrailElementCollection+InterpolationState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrailElementCollection_InterpolationState {
    pub segmentIndex: i32,
    pub segmentLerp: f32,
}
#[cfg(feature = "TrailElementCollection+InterpolationState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TrailElementCollection_InterpolationState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TrailElementCollection/InterpolationState";
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
#[cfg(feature = "TrailElementCollection+InterpolationState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::TrailElementCollection_InterpolationState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "TrailElementCollection+InterpolationState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::TrailElementCollection_InterpolationState {
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
#[cfg(feature = "TrailElementCollection+InterpolationState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::TrailElementCollection_InterpolationState {
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
#[cfg(feature = "TrailElementCollection+InterpolationState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::TrailElementCollection_InterpolationState {
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
