#[cfg(feature = "UnityEngine+XR+MeshGenerationResult")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MeshGenerationResult {
    pub _MeshId_k__BackingField: crate::UnityEngine::XR::MeshId,
    pub _Mesh_k__BackingField: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    pub _MeshCollider_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::MeshCollider,
    >,
    pub _Status_k__BackingField: crate::UnityEngine::XR::MeshGenerationStatus,
    pub _Attributes_k__BackingField: crate::UnityEngine::XR::MeshVertexAttributes,
    pub _Timestamp_k__BackingField: u64,
    pub _Position_k__BackingField: crate::UnityEngine::Vector3,
    pub _Rotation_k__BackingField: crate::UnityEngine::Quaternion,
    pub _Scale_k__BackingField: crate::UnityEngine::Vector3,
}
#[cfg(feature = "UnityEngine+XR+MeshGenerationResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::MeshGenerationResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR";
    const CLASS_NAME: &'static str = "MeshGenerationResult";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::XR::MeshGenerationResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::XR::MeshGenerationResult {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::XR::MeshGenerationResult {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::XR::MeshGenerationResult {
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
#[cfg(feature = "UnityEngine+XR+MeshGenerationResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::XR::MeshGenerationResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+MeshGenerationResult")]
impl crate::UnityEngine::XR::MeshGenerationResult {
    pub fn Equals_Il2CppObject0(
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
    pub fn Equals_MeshGenerationResult1(
        &mut self,
        other: crate::UnityEngine::XR::MeshGenerationResult,
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
    pub fn get_Attributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::XR::MeshVertexAttributes> {
        let __cordl_ret: crate::UnityEngine::XR::MeshVertexAttributes = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Attributes",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Mesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Mesh",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MeshCollider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshCollider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshCollider> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_MeshCollider",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MeshId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::XR::MeshId> {
        let __cordl_ret: crate::UnityEngine::XR::MeshId = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_MeshId",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Position",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Rotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Rotation",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Scale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Scale",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Status(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::XR::MeshGenerationStatus> {
        let __cordl_ret: crate::UnityEngine::XR::MeshGenerationStatus = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Status",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+XR+MeshGenerationResult")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::XR::MeshGenerationResult>>
for crate::UnityEngine::XR::MeshGenerationResult {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::XR::MeshGenerationResult> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+XR+MeshGenerationResult")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::XR::MeshGenerationResult>>
for crate::UnityEngine::XR::MeshGenerationResult {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::XR::MeshGenerationResult> {
        todo!()
    }
}
