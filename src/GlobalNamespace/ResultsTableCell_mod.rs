#[cfg(feature = "ResultsTableCell")]
#[repr(C)]
#[derive(Debug)]
pub struct ResultsTableCell {
    __cordl_parent: crate::GlobalNamespace::TableCellWithSeparator,
    pub _border: *mut crate::UnityEngine::GameObject,
    pub _orderText: *mut crate::TMPro::TextMeshProUGUI,
    pub _nameText: *mut crate::TMPro::TextMeshProUGUI,
    pub _scoreText: *mut crate::TMPro::TextMeshProUGUI,
    pub _rankText: *mut crate::TMPro::TextMeshProUGUI,
}
#[cfg(feature = "ResultsTableCell")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ResultsTableCell => ""
    ."ResultsTableCell"
);
#[cfg(feature = "ResultsTableCell")]
impl std::ops::Deref for crate::GlobalNamespace::ResultsTableCell {
    type Target = crate::GlobalNamespace::TableCellWithSeparator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ResultsTableCell")]
impl std::ops::DerefMut for crate::GlobalNamespace::ResultsTableCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ResultsTableCell")]
impl crate::GlobalNamespace::ResultsTableCell {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetData(
        &mut self,
        order: i32,
        connectedPlayer: *mut crate::GlobalNamespace::IConnectedPlayer,
        levelCompletionResults: *mut crate::GlobalNamespace::LevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (order, connectedPlayer, levelCompletionResults))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ResultsTableCell")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ResultsTableCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
