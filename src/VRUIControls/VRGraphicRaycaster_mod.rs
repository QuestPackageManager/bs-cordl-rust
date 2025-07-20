#[cfg(feature = "VRUIControls+VRGraphicRaycaster")]
#[repr(C)]
#[derive(Debug)]
pub struct VRGraphicRaycaster {
    __cordl_parent: crate::UnityEngine::EventSystems::BaseRaycaster,
    pub _blockingMask: crate::UnityEngine::LayerMask,
    pub _physicsRaycaster: quest_hook::libil2cpp::Gc<
        crate::VRUIControls::PhysicsRaycasterWithCache,
    >,
    pub _canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    pub _raycastResults: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult,
        >,
    >,
    pub _curvedCanvasSettingsHelper: quest_hook::libil2cpp::Gc<
        crate::HMUI::CurvedCanvasSettingsHelper,
    >,
}
#[cfg(feature = "VRUIControls+VRGraphicRaycaster")]
unsafe impl quest_hook::libil2cpp::Type for crate::VRUIControls::VRGraphicRaycaster {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "VRUIControls";
    const CLASS_NAME: &'static str = "VRGraphicRaycaster";
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
#[cfg(feature = "VRUIControls+VRGraphicRaycaster")]
impl std::ops::Deref for crate::VRUIControls::VRGraphicRaycaster {
    type Target = crate::UnityEngine::EventSystems::BaseRaycaster;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+VRGraphicRaycaster")]
impl std::ops::DerefMut for crate::VRUIControls::VRGraphicRaycaster {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+VRGraphicRaycaster")]
impl crate::VRUIControls::VRGraphicRaycaster {
    pub const kPhysics3DRaycastDistance: f32 = 6f32;
    #[cfg(feature = "VRUIControls+VRGraphicRaycaster+VRGraphicRaycastResult")]
    pub type VRGraphicRaycastResult = crate::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnEnable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnEnable", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
        resultAppendList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::EventSystems::RaycastResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::PointerEventData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::EventSystems::RaycastResult,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Raycast",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (eventData, resultAppendList))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastCanvas(
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
        ray: crate::UnityEngine::Ray,
        hitDistance: f32,
        curvedUIRadius: f32,
        results: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
                            crate::UnityEngine::Ray,
                            f32,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("RaycastCanvas")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RaycastCanvas", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (canvas, ray, hitDistance, curvedUIRadius, results),
                )?
        };
        Ok(__cordl_ret.into())
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
    pub fn get_eventCamera(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        0usize,
                    >("get_eventCamera")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_eventCamera", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VRUIControls+VRGraphicRaycaster")]
impl quest_hook::libil2cpp::ObjectType for crate::VRUIControls::VRGraphicRaycaster {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VRUIControls+VRGraphicRaycaster+VRGraphicRaycastResult")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VRGraphicRaycaster_VRGraphicRaycastResult {
    pub graphic: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
    pub distance: f32,
    pub position: crate::UnityEngine::Vector3,
    pub insideRootCanvasPosition: crate::UnityEngine::Vector2,
}
#[cfg(feature = "VRUIControls+VRGraphicRaycaster+VRGraphicRaycastResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "VRUIControls";
    const CLASS_NAME: &'static str = "VRGraphicRaycaster/VRGraphicRaycastResult";
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
#[cfg(feature = "VRUIControls+VRGraphicRaycaster+VRGraphicRaycastResult")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "VRUIControls+VRGraphicRaycaster+VRGraphicRaycastResult")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult {
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
#[cfg(feature = "VRUIControls+VRGraphicRaycaster+VRGraphicRaycastResult")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult {
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
#[cfg(feature = "VRUIControls+VRGraphicRaycaster+VRGraphicRaycastResult")]
unsafe impl quest_hook::libil2cpp::Return
for crate::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult {
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
#[cfg(feature = "VRUIControls+VRGraphicRaycaster+VRGraphicRaycastResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "VRUIControls+VRGraphicRaycaster+VRGraphicRaycastResult")]
impl crate::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult {
    pub fn _ctor(
        &mut self,
        graphic: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
        distance: f32,
        position: crate::UnityEngine::Vector3,
        insideRootCanvasPosition: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
                            f32,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector2,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (graphic, distance, position, insideRootCanvasPosition),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
