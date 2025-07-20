#[cfg(feature = "UnityEngine+UIElements+UITKTextHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct UITKTextHandle {
    __cordl_parent: crate::UnityEngine::TextCore::Text::TextHandle,
    pub _MeasuredSizes_k__BackingField: crate::UnityEngine::Vector2,
    pub _RoundedSizes_k__BackingField: crate::UnityEngine::Vector2,
    pub m_TextElement: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::TextElement,
    >,
    pub isOverridingCursor: bool,
    pub currentLinkIDHash: i32,
    pub hasLinkTag: bool,
    pub hasATag: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextHandle")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UITKTextHandle {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "UITKTextHandle";
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
#[cfg(feature = "UnityEngine+UIElements+UITKTextHandle")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UITKTextHandle {
    type Target = crate::UnityEngine::TextCore::Text::TextHandle;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextHandle")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UITKTextHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextHandle")]
impl crate::UnityEngine::UIElements::UITKTextHandle {
    pub fn ATagOnPointerMove(
        &mut self,
        pme: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerMoveEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::PointerMoveEvent,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ATagOnPointerMove")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "ATagOnPointerMove", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pme))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ATagOnPointerOut(
        &mut self,
        _cordl__: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::PointerOutEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::PointerOutEvent,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ATagOnPointerOut")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "ATagOnPointerOut", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (_cordl__))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ATagOnPointerOver(
        &mut self,
        _cordl__: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::PointerOverEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::PointerOverEvent,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ATagOnPointerOver")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "ATagOnPointerOver", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (_cordl__))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ATagOnPointerUp(
        &mut self,
        pue: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerUpEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::PointerUpEvent,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ATagOnPointerUp")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "ATagOnPointerUp", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeTextHeight(
        &mut self,
        textToMeasure: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        width: f32,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    f32,
                    f32,
                ),
                f32,
                3usize,
            >("ComputeTextHeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "ComputeTextHeight", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked(self, (textToMeasure, width, height))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeTextWidth(
        &mut self,
        textToMeasure: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        wordWrap: bool,
        width: f32,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                    f32,
                    f32,
                ),
                f32,
                4usize,
            >("ComputeTextWidth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "ComputeTextWidth", 4usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked(self, (textToMeasure, wordWrap, width, height))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertUssToTextGenerationSettings(
        &mut self,
        tgs: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ConvertUssToTextGenerationSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ConvertUssToTextGenerationSettings", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tgs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextEffectPadding(
        &mut self,
        fontAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::FontAsset,
                >),
                f32,
                1usize,
            >("GetTextEffectPadding")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "GetTextEffectPadding",
                    1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (fontAsset))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextOverflowMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::Text::TextOverflowMode,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::TextCore::Text::TextOverflowMode,
                0usize,
            >("GetTextOverflowMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "GetTextOverflowMode",
                    0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::TextCore::Text::TextOverflowMode = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleATag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("HandleATag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "HandleATag", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLinkTag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("HandleLinkTag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "HandleLinkTag", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LinkTagOnPointerDown(
        &mut self,
        pde: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerDownEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::PointerDownEvent,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LinkTagOnPointerDown")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "LinkTagOnPointerDown",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pde))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LinkTagOnPointerMove(
        &mut self,
        pme: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerMoveEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::PointerMoveEvent,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LinkTagOnPointerMove")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "LinkTagOnPointerMove",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pme))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LinkTagOnPointerOut(
        &mut self,
        poe: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerOutEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::PointerOutEvent,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LinkTagOnPointerOut")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "LinkTagOnPointerOut",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (poe))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LinkTagOnPointerUp(
        &mut self,
        pue: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerUpEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::PointerUpEvent,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LinkTagOnPointerUp")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "LinkTagOnPointerUp", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        te: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (te))?;
        Ok(__cordl_object.into())
    }
    pub fn TextLibraryCanElide(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("TextLibraryCanElide")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "TextLibraryCanElide",
                    0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
                0usize,
            >("Update")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "Update", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextInfo,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        te: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (te))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_MeasuredSizes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Vector2, 0usize>("get_MeasuredSizes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "get_MeasuredSizes", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_RoundedSizes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Vector2, 0usize>("get_RoundedSizes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "get_RoundedSizes", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_MeasuredSizes(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector2),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_MeasuredSizes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "set_MeasuredSizes", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_RoundedSizes(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UITKTextHandle as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector2),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_RoundedSizes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UITKTextHandle as
                    quest_hook::libil2cpp::Type > ::class(), "set_RoundedSizes", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UITKTextHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
