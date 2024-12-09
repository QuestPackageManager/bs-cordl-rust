#[cfg(feature = "TMPro+TMP_DefaultControls")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_DefaultControls {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "TMPro+TMP_DefaultControls")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_DefaultControls => "TMPro"
    ."TMP_DefaultControls"
);
#[cfg(feature = "TMPro+TMP_DefaultControls")]
impl std::ops::Deref for crate::TMPro::TMP_DefaultControls {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_DefaultControls")]
impl std::ops::DerefMut for crate::TMPro::TMP_DefaultControls {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_DefaultControls")]
impl crate::TMPro::TMP_DefaultControls {
    pub const kThickHeight: f32 = 30f32;
    pub const kThinHeight: f32 = 20f32;
    pub const kWidth: f32 = 160f32;
    #[cfg(feature = "TMPro+TMP_DefaultControls+Resources")]
    pub type Resources = crate::TMPro::TMP_DefaultControls_Resources;
}
#[cfg(feature = "TMPro+TMP_DefaultControls")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_DefaultControls {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+TMP_DefaultControls+Resources")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TMP_DefaultControls_Resources {
    pub standard: *mut crate::UnityEngine::Sprite,
    pub background: *mut crate::UnityEngine::Sprite,
    pub inputField: *mut crate::UnityEngine::Sprite,
    pub knob: *mut crate::UnityEngine::Sprite,
    pub checkmark: *mut crate::UnityEngine::Sprite,
    pub dropdown: *mut crate::UnityEngine::Sprite,
    pub mask: *mut crate::UnityEngine::Sprite,
}
#[cfg(feature = "TMPro+TMP_DefaultControls+Resources")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_DefaultControls_Resources => "TMPro"
    ."TMP_DefaultControls/Resources"
);
#[cfg(feature = "TMPro+TMP_DefaultControls+Resources")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::TMPro::TMP_DefaultControls_Resources {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_DefaultControls+Resources")]
impl crate::TMPro::TMP_DefaultControls_Resources {}
