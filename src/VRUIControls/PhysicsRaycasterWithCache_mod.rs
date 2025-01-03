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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::VRUIControls::PhysicsRaycasterWithCache =>
    "VRUIControls"."PhysicsRaycasterWithCache"
);
#[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache")]
impl std::ops::Deref for crate::VRUIControls::PhysicsRaycasterWithCache {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache")]
impl std::ops::DerefMut for crate::VRUIControls::PhysicsRaycasterWithCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Raycast", (ray, hitInfo, maxDistance, layerMask))?;
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
#[derive(Debug, Clone, Default)]
pub struct PhysicsRaycasterWithCache_CachedRaycast {
    pub wasHit: bool,
    pub ray: crate::UnityEngine::Ray,
    pub hitInfo: crate::UnityEngine::RaycastHit,
    pub maxDistance: f32,
    pub layerMask: i32,
}
#[cfg(feature = "VRUIControls+PhysicsRaycasterWithCache+CachedRaycast")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::VRUIControls::PhysicsRaycasterWithCache_CachedRaycast => "VRUIControls"
    ."PhysicsRaycasterWithCache/CachedRaycast"
);
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
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (wasHit, ray, hitInfo, maxDistance, layerMask),
        )?;
        Ok(__cordl_ret.into())
    }
}
