#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridViewAnimator")]
#[repr(C)]
#[derive(Debug)]
pub struct AnnotatedBeatmapLevelCollectionsGridViewAnimator {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _viewportTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    pub _contentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    pub _transitionDuration: f32,
    pub _easeType: crate::GlobalNamespace::EaseType,
    pub _padding: f32,
    pub _tweeningManager: quest_hook::libil2cpp::Gc<
        crate::Tweening::TimeTweeningManager,
    >,
    pub _columnWidth: f32,
    pub _rowHeight: f32,
    pub _visibleColumnCount: i32,
    pub _columnCount: i32,
    pub _rowCount: i32,
    pub _selectedColumn: i32,
    pub _selectedRow: i32,
    pub _viewportSizeTween: quest_hook::libil2cpp::Gc<crate::Tweening::Vector2Tween>,
    pub _contentPositionTween: quest_hook::libil2cpp::Gc<crate::Tweening::Vector2Tween>,
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridViewAnimator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridViewAnimator => ""
    ."AnnotatedBeatmapLevelCollectionsGridViewAnimator"
);
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridViewAnimator")]
impl std::ops::Deref
for crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridViewAnimator {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridViewAnimator")]
impl std::ops::DerefMut
for crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridViewAnimator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridViewAnimator")]
impl crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridViewAnimator {
    pub fn AnimateClose(
        &mut self,
        selectedColumn: i32,
        selectedRow: i32,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AnimateClose", (selectedColumn, selectedRow, animated))?;
        Ok(__cordl_ret.into())
    }
    pub fn AnimateOpen(
        &mut self,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AnimateOpen", (animated))?;
        Ok(__cordl_ret.into())
    }
    pub fn DespawnAllActiveTweens(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DespawnAllActiveTweens", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContentXOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetContentXOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContentYOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetContentYOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        columnWidth: f32,
        rowHeight: f32,
        columnCount: i32,
        rowCount: i32,
        visibleColumnCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (columnWidth, rowHeight, columnCount, rowCount, visibleColumnCount),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScrollToRowIdxInstant(
        &mut self,
        selectedColumn: i32,
        selectedRow: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScrollToRowIdxInstant", (selectedColumn, selectedRow))?;
        Ok(__cordl_ret.into())
    }
    pub fn _AnimateClose_b__19_0(
        &mut self,
        _cordl_size: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AnimateClose>b__19_0", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn _AnimateClose_b__19_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AnimateClose>b__19_1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _AnimateClose_b__19_2(
        &mut self,
        pos: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AnimateClose>b__19_2", (pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn _AnimateClose_b__19_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AnimateClose>b__19_3", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _AnimateOpen_b__18_0(
        &mut self,
        _cordl_size: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AnimateOpen>b__18_0", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn _AnimateOpen_b__18_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AnimateOpen>b__18_1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _AnimateOpen_b__18_2(
        &mut self,
        pos: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AnimateOpen>b__18_2", (pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn _AnimateOpen_b__18_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AnimateOpen>b__18_3", ())?;
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
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridViewAnimator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridViewAnimator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
