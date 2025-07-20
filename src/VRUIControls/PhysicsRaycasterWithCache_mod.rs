#[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache")]
#[repr(C)]
#[derive(Debug)]
pub struct PhysicsRaycasterWithCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cachedRaycasts: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::VRUIControls::PhysicsRaycasterWithCache_CachedRaycast,
        >,
    >,
    pub _lastFrameCount: i32,
}
#[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache")]
unsafe impl quest_hook::libil2cpp::Type
for crate::VRUIControls::PhysicsRaycasterWithCache {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "VRUIControls";
    const CLASS_NAME: &'static str = "PhysicsRaycasterWithCache";
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
#[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache")]
impl std::ops::Deref for crate::VRUIControls::PhysicsRaycasterWithCache {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache")]
impl std::ops::DerefMut for crate::VRUIControls::PhysicsRaycasterWithCache {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache")]
impl crate::VRUIControls::PhysicsRaycasterWithCache {
    #[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache+CachedRaycast")]
    pub type CachedRaycast = crate::VRUIControls::PhysicsRaycasterWithCache_CachedRaycast;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Raycast(
        &mut self,
        ray: crate::UnityEngine::Ray,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Ray,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RaycastHit,
                            >,
                            f32,
                            i32,
                        ),
                        bool,
                        4usize,
                    >("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Raycast", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (ray, hitInfo, maxDistance, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache")]
impl quest_hook::libil2cpp::ObjectType
for crate::VRUIControls::PhysicsRaycasterWithCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache+CachedRaycast")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhysicsRaycasterWithCache_CachedRaycast {
    pub wasHit: bool,
    pub ray: crate::UnityEngine::Ray,
    pub hitInfo: crate::UnityEngine::RaycastHit,
    pub maxDistance: f32,
    pub layerMask: i32,
}
#[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache+CachedRaycast")]
unsafe impl quest_hook::libil2cpp::Type
for crate::VRUIControls::PhysicsRaycasterWithCache_CachedRaycast {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "VRUIControls";
    const CLASS_NAME: &'static str = "PhysicsRaycasterWithCache/CachedRaycast";
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
#[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache+CachedRaycast")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::VRUIControls::PhysicsRaycasterWithCache_CachedRaycast {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache+CachedRaycast")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::VRUIControls::PhysicsRaycasterWithCache_CachedRaycast {
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
#[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache+CachedRaycast")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::VRUIControls::PhysicsRaycasterWithCache_CachedRaycast {
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
#[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache+CachedRaycast")]
unsafe impl quest_hook::libil2cpp::Return
for crate::VRUIControls::PhysicsRaycasterWithCache_CachedRaycast {
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
#[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache+CachedRaycast")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::VRUIControls::PhysicsRaycasterWithCache_CachedRaycast {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache+CachedRaycast")]
impl crate::VRUIControls::PhysicsRaycasterWithCache_CachedRaycast {
    pub fn _ctor(
        &mut self,
        wasHit: bool,
        ray: crate::UnityEngine::Ray,
        hitInfo: crate::UnityEngine::RaycastHit,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            bool,
                            crate::UnityEngine::Ray,
                            crate::UnityEngine::RaycastHit,
                            f32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (wasHit, ray, hitInfo, maxDistance, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
}
