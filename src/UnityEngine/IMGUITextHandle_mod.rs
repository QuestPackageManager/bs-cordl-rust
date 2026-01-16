#[cfg(feature = "cordl_class_UnityEngine+IMGUITextHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct IMGUITextHandle {
    __cordl_parent: crate::UnityEngine::TextCore::Text::TextHandle,
    pub tuple: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::LinkedListNode_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::IMGUITextHandle_TextHandleTuple>,
        >,
    >,
    pub isCachedOnNative: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+IMGUITextHandle")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::IMGUITextHandle {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "IMGUITextHandle";
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
#[cfg(feature = "UnityEngine+IMGUITextHandle")]
impl std::ops::Deref for crate::UnityEngine::IMGUITextHandle {
    type Target = crate::UnityEngine::TextCore::Text::TextHandle;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+IMGUITextHandle")]
impl std::ops::DerefMut for crate::UnityEngine::IMGUITextHandle {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+IMGUITextHandle")]
impl crate::UnityEngine::IMGUITextHandle {
    #[cfg(feature = "UnityEngine+IMGUITextHandle+TextHandleTuple")]
    pub type TextHandleTuple = crate::UnityEngine::IMGUITextHandle_TextHandleTuple;
    pub fn ClearUnusedTextHandles() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "ClearUnusedTextHandles",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearUnusedTextHandles",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertGUIStyleToGenerationSettings(
        settings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        textColor: crate::UnityEngine::Color,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        rect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
                        crate::UnityEngine::Color,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Rect,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "ConvertGUIStyleToGenerationSettings"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConvertGUIStyleToGenerationSettings",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (settings, style, textColor, text, rect))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EmptyManagedCache() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "EmptyManagedCache",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EmptyManagedCache",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLineHeight(
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>),
                        f32,
                        1usize,
                    >("GetLineHeight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLineHeight", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (style))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPreferredSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::Vector2, 0usize>("GetPreferredSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetPreferredSize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextHandle_GUIStyle_Rect_Il2CppString_Color32_0(
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        position: crate::UnityEngine::Rect,
        content: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        textColor: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::IMGUITextHandle>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
                        crate::UnityEngine::Rect,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Color32,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::IMGUITextHandle>, 4usize>(
                        "GetTextHandle",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTextHandle",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::IMGUITextHandle> = unsafe {
            cordl_method_info.invoke_unchecked((), (style, position, content, textColor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextHandle_GUIStyle_Rect_Il2CppString_Color32_ByRefMut1(
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        position: crate::UnityEngine::Rect,
        content: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        textColor: crate::UnityEngine::Color32,
        isCached: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::IMGUITextHandle>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
                        crate::UnityEngine::Rect,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Color32,
                        quest_hook::libil2cpp::ByRefMut<bool>,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::IMGUITextHandle>, 5usize>(
                        "GetTextHandle",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTextHandle",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::IMGUITextHandle> = unsafe {
            cordl_method_info
                .invoke_unchecked((), (style, position, content, textColor, isCached))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextHandle_TextGenerationSettings__cordl_bool_ByRefMut2(
        settings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        isCalledFromNative: bool,
        isCached: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::IMGUITextHandle>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                        >,
                        bool,
                        quest_hook::libil2cpp::ByRefMut<bool>,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::IMGUITextHandle>, 3usize>(
                        "GetTextHandle",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTextHandle",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::IMGUITextHandle> = unsafe {
            cordl_method_info.invoke_unchecked((), (settings, isCalledFromNative, isCached))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LegacyClippingToNewOverflow(
        clipping: crate::UnityEngine::TextClipping,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::Text::TextOverflowMode> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::TextClipping),
                        crate::UnityEngine::TextCore::Text::TextOverflowMode,
                        1usize,
                    >("LegacyClippingToNewOverflow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LegacyClippingToNewOverflow", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::Text::TextOverflowMode =
            unsafe { cordl_method_info.invoke_unchecked((), (clipping))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ShouldCleanup(
        currentTime: f32,
        lastTime: f32,
        cleanupThreshold: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32, f32, f32), bool, 3usize>("ShouldCleanup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ShouldCleanup",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (currentTime, lastTime, cleanupThreshold))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+IMGUITextHandle")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::IMGUITextHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+IMGUITextHandle+TextHandleTuple")]
#[repr(C)]
#[derive(Debug)]
pub struct IMGUITextHandle_TextHandleTuple {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub lastTimeUsed: f32,
    pub hashCode: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+IMGUITextHandle+TextHandleTuple")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::IMGUITextHandle_TextHandleTuple {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "IMGUITextHandle/TextHandleTuple";
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
#[cfg(feature = "UnityEngine+IMGUITextHandle+TextHandleTuple")]
impl std::ops::Deref for crate::UnityEngine::IMGUITextHandle_TextHandleTuple {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+IMGUITextHandle+TextHandleTuple")]
impl std::ops::DerefMut for crate::UnityEngine::IMGUITextHandle_TextHandleTuple {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+IMGUITextHandle+TextHandleTuple")]
impl crate::UnityEngine::IMGUITextHandle_TextHandleTuple {
    pub fn New(
        lastTimeUsed: f32,
        hashCode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lastTimeUsed, hashCode))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        lastTimeUsed: f32,
        hashCode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f32, i32), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (lastTimeUsed, hashCode))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+IMGUITextHandle+TextHandleTuple")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::IMGUITextHandle_TextHandleTuple {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
