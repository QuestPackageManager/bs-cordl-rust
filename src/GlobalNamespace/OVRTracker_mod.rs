#[cfg(feature = "OVRTracker")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTracker {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRTracker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTracker => ""."OVRTracker"
);
#[cfg(feature = "OVRTracker")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTracker {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTracker")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTracker")]
impl crate::GlobalNamespace::OVRTracker {
    #[cfg(feature = "OVRTracker+Frustum")]
    pub type Frustum = crate::GlobalNamespace::OVRTracker_Frustum;
    pub fn GetFrustum(
        &mut self,
        tracker: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTracker_Frustum> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRTracker_Frustum = __cordl_object
            .invoke("GetFrustum", (tracker))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPose(
        &mut self,
        tracker: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPose = __cordl_object
            .invoke("GetPose", (tracker))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPoseValid(&mut self, tracker: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetPoseValid", (tracker))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPresent(&mut self, tracker: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetPresent", (tracker))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isPositionTracked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isPositionTracked", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isPresent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isPresent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRTracker")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRTracker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRTracker+Frustum")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct OVRTracker_Frustum {
    pub nearZ: f32,
    pub farZ: f32,
    pub fov: crate::UnityEngine::Vector2,
}
#[cfg(feature = "OVRTracker+Frustum")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTracker_Frustum => ""
    ."OVRTracker/Frustum"
);
#[cfg(feature = "OVRTracker+Frustum")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRTracker_Frustum {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRTracker+Frustum")]
impl crate::GlobalNamespace::OVRTracker_Frustum {}
