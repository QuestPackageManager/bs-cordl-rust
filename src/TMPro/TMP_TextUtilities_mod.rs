#[cfg(feature = "TMPro+TMP_TextUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_TextUtilities {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "TMPro+TMP_TextUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_TextUtilities => "TMPro"
    ."TMP_TextUtilities"
);
#[cfg(feature = "TMPro+TMP_TextUtilities")]
impl std::ops::Deref for crate::TMPro::TMP_TextUtilities {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn DistanceToLine(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DistanceToLine", (a, b, point))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindIntersectingCharacter(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        visibleOnly: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindIntersectingCharacter", (text, position, camera, visibleOnly))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindIntersectingLine(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindIntersectingLine", (text, position, camera))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindIntersectingLink(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindIntersectingLink", (text, position, camera))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindIntersectingWord(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindIntersectingWord", (text, position, camera))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindNearestCharacter(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        visibleOnly: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindNearestCharacter", (text, position, camera, visibleOnly))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindNearestCharacterOnLine(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        line: i32,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        visibleOnly: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FindNearestCharacterOnLine",
                (text, position, line, camera, visibleOnly),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FindNearestLine(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindNearestLine", (text, position, camera))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindNearestLink(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindNearestLink", (text, position, camera))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindNearestWord(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindNearestWord", (text, position, camera))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCursorIndexFromPosition_ByRefMut1(
        textComponent: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        cursor: quest_hook::libil2cpp::ByRefMut<crate::TMPro::CaretPosition>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetCursorIndexFromPosition",
                (textComponent, position, camera, cursor),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCursorIndexFromPosition_Gc_Vector3_Gc0(
        textComponent: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCursorIndexFromPosition", (textComponent, position, camera))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHashCode", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSimpleHashCode(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSimpleHashCode", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSimpleHashCodeLowercase(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSimpleHashCodeLowercase", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn HexToInt(hex: char) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HexToInt", (hex))?;
        Ok(__cordl_ret.into())
    }
    pub fn IntersectLinePlane(
        line: crate::TMPro::TMP_TextUtilities_LineSegment,
        point: crate::UnityEngine::Vector3,
        normal: crate::UnityEngine::Vector3,
        intersectingPoint: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IntersectLinePlane", (line, point, normal, intersectingPoint))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsIntersectingRectTransform(
        rectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsIntersectingRectTransform", (rectTransform, position, camera))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointIntersectRectangle(
        m: crate::UnityEngine::Vector3,
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
        c: crate::UnityEngine::Vector3,
        d: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointIntersectRectangle", (m, a, b, c, d))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScreenPointToWorldPointInRectangle(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        screenPoint: crate::UnityEngine::Vector2,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        worldPoint: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ScreenPointToWorldPointInRectangle",
                (transform, screenPoint, cam, worldPoint),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StringHexToInt(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StringHexToInt", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToLowerFast(c: char) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToLowerFast", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperASCIIFast(c: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUpperASCIIFast", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperFast(c: char) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUpperFast", (c))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "TMPro+TMP_TextUtilities+LineSegment")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
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
        Ok(__cordl_ret.into())
    }
}
