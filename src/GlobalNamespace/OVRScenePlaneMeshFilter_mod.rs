#[cfg(feature = "OVRScenePlaneMeshFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRScenePlaneMeshFilter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _meshFilter: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshFilter>,
    pub _mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    pub _jobHandle: crate::System::Nullable_1<crate::Unity::Jobs::JobHandle>,
    pub _meshRequested: bool,
    pub _boundary: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector2>,
    pub _triangles: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(feature = "OVRScenePlaneMeshFilter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRScenePlaneMeshFilter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRScenePlaneMeshFilter";
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
#[cfg(feature = "OVRScenePlaneMeshFilter")]
impl std::ops::Deref for crate::GlobalNamespace::OVRScenePlaneMeshFilter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRScenePlaneMeshFilter")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRScenePlaneMeshFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRScenePlaneMeshFilter")]
impl crate::GlobalNamespace::OVRScenePlaneMeshFilter {
    #[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob")]
    pub type TriangulateBoundaryJob = crate::GlobalNamespace::OVRScenePlaneMeshFilter_TriangulateBoundaryJob;
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
    pub fn RequestMeshGeneration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestMeshGeneration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleMeshGeneration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScheduleMeshGeneration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
#[cfg(feature = "OVRScenePlaneMeshFilter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRScenePlaneMeshFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRScenePlaneMeshFilter_TriangulateBoundaryJob {
    pub Boundary: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector2>,
    pub Triangles: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRScenePlaneMeshFilter_TriangulateBoundaryJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TriangulateBoundaryJob";
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
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRScenePlaneMeshFilter_TriangulateBoundaryJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRScenePlaneMeshFilter_TriangulateBoundaryJob {
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
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRScenePlaneMeshFilter_TriangulateBoundaryJob {
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
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRScenePlaneMeshFilter_TriangulateBoundaryJob {
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
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRScenePlaneMeshFilter_TriangulateBoundaryJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob")]
impl crate::GlobalNamespace::OVRScenePlaneMeshFilter_TriangulateBoundaryJob {
    #[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob+NList")]
    pub type NList = crate::GlobalNamespace::TriangulateBoundaryJob_OVRScenePlaneMeshFilter_NList;
    pub fn Cross(
        a: crate::UnityEngine::Vector2,
        b: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Cross", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PointInTriangle(
        p: crate::UnityEngine::Vector2,
        a: crate::UnityEngine::Vector2,
        b: crate::UnityEngine::Vector2,
        c: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointInTriangle", (p, a, b, c))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob")]
impl AsRef<crate::Unity::Jobs::IJob>
for crate::GlobalNamespace::OVRScenePlaneMeshFilter_TriangulateBoundaryJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob")]
impl AsMut<crate::Unity::Jobs::IJob>
for crate::GlobalNamespace::OVRScenePlaneMeshFilter_TriangulateBoundaryJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob+NList")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TriangulateBoundaryJob_OVRScenePlaneMeshFilter_NList {
    pub _Count_k__BackingField: i32,
    pub _data: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob+NList")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TriangulateBoundaryJob_OVRScenePlaneMeshFilter_NList {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NList";
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
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob+NList")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::TriangulateBoundaryJob_OVRScenePlaneMeshFilter_NList {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob+NList")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::TriangulateBoundaryJob_OVRScenePlaneMeshFilter_NList {
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
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob+NList")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::TriangulateBoundaryJob_OVRScenePlaneMeshFilter_NList {
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
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob+NList")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::TriangulateBoundaryJob_OVRScenePlaneMeshFilter_NList {
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
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob+NList")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::TriangulateBoundaryJob_OVRScenePlaneMeshFilter_NList {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob+NList")]
impl crate::GlobalNamespace::TriangulateBoundaryJob_OVRScenePlaneMeshFilter_NList {
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
    pub fn GetAt(&mut self, index: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetAt",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveAt",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        capacity: i32,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (capacity, allocator),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Count",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Count(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Count",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob+NList")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::TriangulateBoundaryJob_OVRScenePlaneMeshFilter_NList {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "OVRScenePlaneMeshFilter+TriangulateBoundaryJob+NList")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::TriangulateBoundaryJob_OVRScenePlaneMeshFilter_NList {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
