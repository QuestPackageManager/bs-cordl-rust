#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ToolsInfo {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _paintBrushSize: f32,
    pub _paintBrushOpacity: f32,
    pub _paintIntValue: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub _paintFloatValue: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<f32>,
    >,
    pub _paintStringValue: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _lastAttributesGeoID: i32,
    pub _lastAttributesPartID: i32,
    pub _lastAttributeNodeName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _lastAttributeName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _brushHandleColor: crate::UnityEngine::Color,
    pub _affectedAreaPaintColor: crate::UnityEngine::Color,
    pub _liveUpdate: bool,
    pub _isPainting: bool,
    pub _editPointBoxSize: f32,
    pub _editPointBoxUnselectedColor: crate::UnityEngine::Color,
    pub _editPointBoxSelectedColor: crate::UnityEngine::Color,
    pub _recacheRequired: bool,
    pub _paintMergeMode: crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMergeMode,
    pub _showOnlyEditGeometry: bool,
    pub _alwaysCookUpstream: bool,
    pub _paintMeshVisiblity: crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMeshVisibility,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HEU_ToolsInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_ToolsInfo";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_ToolsInfo {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_ToolsInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo")]
impl crate::HoudiniEngineUnity::HEU_ToolsInfo {
    #[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMergeMode")]
    pub type PaintMergeMode = crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMergeMode;
    #[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMeshVisibility")]
    pub type PaintMeshVisibility = crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMeshVisibility;
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ToolsInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::HEU_ToolsInfo,
                        >),
                        bool,
                        1usize,
                    >("IsEquivalentTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsEquivalentTo", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_ToolsInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ToolsInfo>,
    >,
> for crate::HoudiniEngineUnity::HEU_ToolsInfo {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ToolsInfo>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ToolsInfo>,
    >,
> for crate::HoudiniEngineUnity::HEU_ToolsInfo {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ToolsInfo>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMergeMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_ToolsInfo_PaintMergeMode {
    #[default]
    ADD = 1i32,
    MULTIPLY = 3i32,
    REPLACE = 0i32,
    SUBTRACT = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMergeMode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMergeMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_ToolsInfo/PaintMergeMode";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMergeMode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMergeMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMergeMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMergeMode {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMergeMode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMergeMode {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMergeMode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMergeMode {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMeshVisibility")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_ToolsInfo_PaintMeshVisibility {
    #[default]
    AUTO = 0i32,
    HIDE = 2i32,
    SHOW = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMeshVisibility")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMeshVisibility {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_ToolsInfo/PaintMeshVisibility";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMeshVisibility")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMeshVisibility {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMeshVisibility")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMeshVisibility {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMeshVisibility")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMeshVisibility {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ToolsInfo+PaintMeshVisibility")]
unsafe impl quest_hook::libil2cpp::Return
for crate::HoudiniEngineUnity::HEU_ToolsInfo_PaintMeshVisibility {
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
