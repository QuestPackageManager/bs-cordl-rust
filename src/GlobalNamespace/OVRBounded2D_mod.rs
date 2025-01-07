#[cfg(feature = "OVRBounded2D")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRBounded2D {
    pub _Handle_k__BackingField: u64,
}
#[cfg(feature = "OVRBounded2D")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRBounded2D {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRBounded2D";
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
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRBounded2D {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRBounded2D {
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
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRBounded2D {
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
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRBounded2D {
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
#[cfg(feature = "OVRBounded2D")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRBounded2D {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRBounded2D")]
impl crate::GlobalNamespace::OVRBounded2D {
    pub fn ConvertRect(
        &mut self,
        openXrRect: crate::GlobalNamespace::OVRPlugin_Rectf,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ConvertRect",
            (openXrRect),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
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
    pub fn Equals_OVRBounded2D0(
        &mut self,
        other: crate::GlobalNamespace::OVRBounded2D,
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
    pub fn IOVRAnchorComponent_OVRBounded2D__FromAnchor(
        &mut self,
        anchor: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRBounded2D> {
        let __cordl_ret: crate::GlobalNamespace::OVRBounded2D = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRBounded2D>.FromAnchor",
            (anchor),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IOVRAnchorComponent_OVRBounded2D__SetEnabledAsync(
        &mut self,
        enabled: bool,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRBounded2D>.SetEnabledAsync",
            (enabled, timeout),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IOVRAnchorComponent_OVRBounded2D__get_Handle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRBounded2D>.get_Handle",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IOVRAnchorComponent_OVRBounded2D__get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRBounded2D>.get_Type",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetBoundaryPoints(
        &mut self,
        positions: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetBoundaryPoints",
            (positions),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetBoundaryPointsCount(
        &mut self,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetBoundaryPointsCount",
            (count),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        anchor: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (anchor),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BoundingBox(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_BoundingBox",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Handle(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Handle",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsEnabled",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsNull",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Type",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::GlobalNamespace::OVRBounded2D,
        rhs: crate::GlobalNamespace::OVRBounded2D,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::GlobalNamespace::OVRBounded2D,
        rhs: crate::GlobalNamespace::OVRBounded2D,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRBounded2D")]
impl AsRef<
    crate::GlobalNamespace::IOVRAnchorComponent_1<crate::GlobalNamespace::OVRBounded2D>,
> for crate::GlobalNamespace::OVRBounded2D {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IOVRAnchorComponent_1<
        crate::GlobalNamespace::OVRBounded2D,
    > {
        todo!()
    }
}
#[cfg(feature = "OVRBounded2D")]
impl AsMut<
    crate::GlobalNamespace::IOVRAnchorComponent_1<crate::GlobalNamespace::OVRBounded2D>,
> for crate::GlobalNamespace::OVRBounded2D {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IOVRAnchorComponent_1<
        crate::GlobalNamespace::OVRBounded2D,
    > {
        todo!()
    }
}
#[cfg(feature = "OVRBounded2D")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::OVRBounded2D>>
for crate::GlobalNamespace::OVRBounded2D {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::GlobalNamespace::OVRBounded2D> {
        todo!()
    }
}
#[cfg(feature = "OVRBounded2D")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::OVRBounded2D>>
for crate::GlobalNamespace::OVRBounded2D {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::GlobalNamespace::OVRBounded2D> {
        todo!()
    }
}
