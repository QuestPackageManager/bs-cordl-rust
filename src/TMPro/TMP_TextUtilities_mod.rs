#[cfg(feature = "cordl_class_TMPro+TMP_TextUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_TextUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_TMPro+TMP_TextUtilities")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_TextUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_TextUtilities";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "TMPro+TMP_TextUtilities")]
impl std::ops::Deref for crate::TMPro::TMP_TextUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_TextUtilities")]
impl std::ops::DerefMut for crate::TMPro::TMP_TextUtilities {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                    ), f32, 3usize>("DistanceToLine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DistanceToLine",
                            3usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (a, b, point))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindIntersectingCharacter(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        visibleOnly: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        bool,
                    ), i32, 4usize>("FindIntersectingCharacter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindIntersectingCharacter",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (text, position, camera, visibleOnly))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindIntersectingLine(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    ), i32, 3usize>("FindIntersectingLine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindIntersectingLine",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (text, position, camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindIntersectingLink(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    ), i32, 3usize>("FindIntersectingLink")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindIntersectingLink",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (text, position, camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindIntersectingWord(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    ), i32, 3usize>("FindIntersectingWord")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindIntersectingWord",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (text, position, camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindNearestCharacter(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        visibleOnly: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        bool,
                    ), i32, 4usize>("FindNearestCharacter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindNearestCharacter",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (text, position, camera, visibleOnly))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindNearestCharacterOnLine(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        line: i32,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        visibleOnly: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
                        crate::UnityEngine::Vector3,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        bool,
                    ), i32, 5usize>("FindNearestCharacterOnLine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindNearestCharacterOnLine",
                            5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (text, position, line, camera, visibleOnly))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindNearestLine(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    ), i32, 3usize>("FindNearestLine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindNearestLine",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (text, position, camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindNearestLink(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    ), i32, 3usize>("FindNearestLink")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindNearestLink",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (text, position, camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindNearestWord(
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    ), i32, 3usize>("FindNearestWord")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindNearestWord",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (text, position, camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCursorIndexFromPosition_ByRefMut1(
        textComponent: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        cursor: quest_hook::libil2cpp::ByRefMut<crate::TMPro::CaretPosition>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        quest_hook::libil2cpp::ByRefMut<crate::TMPro::CaretPosition>,
                    ), i32, 4usize>("GetCursorIndexFromPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCursorIndexFromPosition",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (textComponent, position, camera, cursor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCursorIndexFromPosition_TMP_Text_Vector3_Camera0(
        textComponent: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    ), i32, 3usize>("GetCursorIndexFromPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCursorIndexFromPosition",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (textComponent, position, camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        i32,
                        1usize,
                    >("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHashCode", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (s))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCodeCaseInSensitive(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        u32,
                        1usize,
                    >("GetHashCodeCaseInSensitive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHashCodeCaseInSensitive", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), (s))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSimpleHashCode(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        i32,
                        1usize,
                    >("GetSimpleHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSimpleHashCode", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (s))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSimpleHashCodeLowercase(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        u32,
                        1usize,
                    >("GetSimpleHashCodeLowercase")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSimpleHashCodeLowercase", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), (s))? };
        Ok(__cordl_ret.into())
    }
    pub fn HexToInt(hex: char) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(char), i32, 1usize>("HexToInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "HexToInt",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (hex))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectLinePlane(
        line: crate::TMPro::TMP_TextUtilities_LineSegment,
        point: crate::UnityEngine::Vector3,
        normal: crate::UnityEngine::Vector3,
        intersectingPoint: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::TMPro::TMP_TextUtilities_LineSegment,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                    ), bool, 4usize>("IntersectLinePlane")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectLinePlane",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (line, point, normal, intersectingPoint))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsIntersectingRectTransform(
        rectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        position: crate::UnityEngine::Vector3,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    ), bool, 3usize>("IsIntersectingRectTransform")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsIntersectingRectTransform",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (rectTransform, position, camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn PointIntersectRectangle(
        m: crate::UnityEngine::Vector3,
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
        c: crate::UnityEngine::Vector3,
        d: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                    ), bool, 5usize>("PointIntersectRectangle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PointIntersectRectangle",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (m, a, b, c, d))? };
        Ok(__cordl_ret.into())
    }
    pub fn ScreenPointToWorldPointInRectangle(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        screenPoint: crate::UnityEngine::Vector2,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        worldPoint: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                        crate::UnityEngine::Vector2,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                    ), bool, 4usize>("ScreenPointToWorldPointInRectangle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScreenPointToWorldPointInRectangle",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (transform, screenPoint, cam, worldPoint))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StringHexToInt(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        i32,
                        1usize,
                    >("StringHexToInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StringHexToInt", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (s))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToLowerFast(c: char) -> quest_hook::libil2cpp::Result<char> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(char), char, 1usize>("ToLowerFast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToLowerFast",
                            1usize
                        )
                    })
            });
        let __cordl_ret: char = unsafe { cordl_method_info.invoke_unchecked((), (c))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperASCIIFast(c: u32) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u32), u32, 1usize>("ToUpperASCIIFast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToUpperASCIIFast",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), (c))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperFast(c: char) -> quest_hook::libil2cpp::Result<char> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(char), char, 1usize>("ToUpperFast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToUpperFast",
                            1usize
                        )
                    })
            });
        let __cordl_ret: char = unsafe { cordl_method_info.invoke_unchecked((), (c))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_TMPro+TMP_TextUtilities")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_TextUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_TMPro+TMP_TextUtilities+LineSegment")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct TMP_TextUtilities_LineSegment {
    pub Point1: crate::UnityEngine::Vector3,
    pub Point2: crate::UnityEngine::Vector3,
}
#[cfg(feature = "cordl_class_TMPro+TMP_TextUtilities+LineSegment")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_TextUtilities_LineSegment {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_TextUtilities/LineSegment";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_TMPro+TMP_TextUtilities+LineSegment")]
unsafe impl quest_hook::libil2cpp::Argument for crate::TMPro::TMP_TextUtilities_LineSegment {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_TMPro+TMP_TextUtilities+LineSegment")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::TMPro::TMP_TextUtilities_LineSegment {
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
#[cfg(feature = "cordl_class_TMPro+TMP_TextUtilities+LineSegment")]
unsafe impl quest_hook::libil2cpp::Returned for crate::TMPro::TMP_TextUtilities_LineSegment {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_TMPro+TMP_TextUtilities+LineSegment")]
unsafe impl quest_hook::libil2cpp::Return for crate::TMPro::TMP_TextUtilities_LineSegment {
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
#[cfg(feature = "cordl_class_TMPro+TMP_TextUtilities+LineSegment")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::TMP_TextUtilities_LineSegment {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (p1, p2))? };
        Ok(__cordl_ret.into())
    }
}
