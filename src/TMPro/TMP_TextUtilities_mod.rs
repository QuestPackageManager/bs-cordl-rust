#[cfg(feature = "TMPro+TMP_TextUtilities+LineSegment")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TMP_TextUtilities_LineSegment {
    pub Point1: crate::UnityEngine::Vector3,
    pub Point2: crate::UnityEngine::Vector3,
}
#[cfg(feature = "TMPro+TMP_TextUtilities+LineSegment")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_TextUtilities_LineSegment => "TMPro"
    ."TMP_TextUtilities/LineSegment"
);
#[cfg(feature = "TMPro+TMP_TextUtilities+LineSegment")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::TMPro::TMP_TextUtilities_LineSegment {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_TextUtilities+LineSegment")]
impl crate::TMPro::TMP_TextUtilities_LineSegment {
    pub fn _ctor(
        &mut self,
        p1: crate::UnityEngine::Vector3,
        p2: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (p1, p2),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TMPro+TMP_TextUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_TextUtilities {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "TMPro+TMP_TextUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_TextUtilities => "TMPro"
    ."TMP_TextUtilities"
);
#[cfg(feature = "TMPro+TMP_TextUtilities")]
impl std::ops::Deref for crate::TMPro::TMP_TextUtilities {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_TextUtilities")]
impl std::ops::DerefMut for crate::TMPro::TMP_TextUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_TextUtilities")]
impl crate::TMPro::TMP_TextUtilities {
    pub const k_lookupStringL: &'static str = "-------------------------------- !-#$%&-()*+,-./0123456789:;<=>?@abcdefghijklmnopqrstuvwxyz[-]^_`abcdefghijklmnopqrstuvwxyz{|}~-";
    pub const k_lookupStringU: &'static str = "-------------------------------- !-#$%&-()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[-]^_`ABCDEFGHIJKLMNOPQRSTUVWXYZ{|}~-";
    #[cfg(feature = "TMPro+TMP_TextUtilities+LineSegment")]
    pub type LineSegment = crate::TMPro::TMP_TextUtilities_LineSegment;
}
#[cfg(feature = "TMPro+TMP_TextUtilities")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_TextUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
