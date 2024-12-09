#[cfg(feature = "UnityEngine+SendMouseEvents")]
#[repr(C)]
#[derive(Debug)]
pub struct SendMouseEvents {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+SendMouseEvents")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SendMouseEvents => "UnityEngine"
    ."SendMouseEvents"
);
#[cfg(feature = "UnityEngine+SendMouseEvents")]
impl std::ops::Deref for crate::UnityEngine::SendMouseEvents {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SendMouseEvents")]
impl std::ops::DerefMut for crate::UnityEngine::SendMouseEvents {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SendMouseEvents")]
impl crate::UnityEngine::SendMouseEvents {
    #[cfg(feature = "UnityEngine+SendMouseEvents+HitInfo")]
    pub type HitInfo = crate::UnityEngine::SendMouseEvents_HitInfo;
}
#[cfg(feature = "UnityEngine+SendMouseEvents")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::SendMouseEvents {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+SendMouseEvents+HitInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SendMouseEvents_HitInfo {
    pub target: *mut crate::UnityEngine::GameObject,
    pub camera: *mut crate::UnityEngine::Camera,
}
#[cfg(feature = "UnityEngine+SendMouseEvents+HitInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SendMouseEvents_HitInfo =>
    "UnityEngine"."SendMouseEvents/HitInfo"
);
#[cfg(feature = "UnityEngine+SendMouseEvents+HitInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::SendMouseEvents_HitInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+SendMouseEvents+HitInfo")]
impl crate::UnityEngine::SendMouseEvents_HitInfo {
    pub fn SendMessage(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SendMessage",
            (name),
        )?;
        Ok(__cordl_ret)
    }
}
