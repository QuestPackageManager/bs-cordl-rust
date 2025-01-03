#[cfg(feature = "TMPro+TMP_Compatibility")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_Compatibility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TMPro+TMP_Compatibility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_Compatibility => "TMPro"
    ."TMP_Compatibility"
);
#[cfg(feature = "TMPro+TMP_Compatibility")]
impl std::ops::Deref for crate::TMPro::TMP_Compatibility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_Compatibility")]
impl std::ops::DerefMut for crate::TMPro::TMP_Compatibility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_Compatibility")]
impl crate::TMPro::TMP_Compatibility {
    #[cfg(feature = "TMPro+TMP_Compatibility+AnchorPositions")]
    pub type AnchorPositions = crate::TMPro::TMP_Compatibility_AnchorPositions;
    pub fn ConvertTextAlignmentEnumValues(
        oldValue: crate::TMPro::TextAlignmentOptions,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TextAlignmentOptions> {
        let __cordl_ret: crate::TMPro::TextAlignmentOptions = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertTextAlignmentEnumValues", (oldValue))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_Compatibility")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_Compatibility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+TMP_Compatibility+AnchorPositions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TMP_Compatibility_AnchorPositions {
    BaseLine = 9i32,
    Bottom = 7i32,
    BottomLeft = 6i32,
    BottomRight = 8i32,
    Center = 4i32,
    Left = 3i32,
    None = 10i32,
    Right = 5i32,
    Top = 1i32,
    TopLeft = 0i32,
    TopRight = 2i32,
}
#[cfg(feature = "TMPro+TMP_Compatibility+AnchorPositions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_Compatibility_AnchorPositions =>
    "TMPro"."TMP_Compatibility/AnchorPositions"
);
