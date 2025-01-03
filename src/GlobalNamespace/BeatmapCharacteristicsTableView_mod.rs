#[cfg(feature = "BeatmapCharacteristicsTableView")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCharacteristicsTableView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _tableView: quest_hook::libil2cpp::Gc<crate::HMUI::TableView>,
    pub _cellPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCharacteristicTableCell,
    >,
    pub _cellReuseIdentifier: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _cellWidth: f32,
    pub didSelectCharacteristic: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut crate::GlobalNamespace::BeatmapCharacteristicSO>,
    >,
    pub _beatmapCharacteristicCollection: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCharacteristicCollection,
    >,
    pub _isInitialized: bool,
    pub _selectedColumn: i32,
}
#[cfg(feature = "BeatmapCharacteristicsTableView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapCharacteristicsTableView
    => ""."BeatmapCharacteristicsTableView"
);
#[cfg(feature = "BeatmapCharacteristicsTableView")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapCharacteristicsTableView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicsTableView")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapCharacteristicsTableView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicsTableView")]
impl crate::GlobalNamespace::BeatmapCharacteristicsTableView {
    pub fn CellForIdx(
        &mut self,
        tableView: quest_hook::libil2cpp::Gc<crate::HMUI::TableView>,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::TableCell>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::TableCell> = __cordl_object
            .invoke("CellForIdx", (tableView, idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn CellSize(&mut self, idx: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("CellSize", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleAdditionalContentModelDidInvalidateData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAdditionalContentModelDidInvalidateData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleDidSelectColumnEvent(
        &mut self,
        tableView: quest_hook::libil2cpp::Gc<crate::HMUI::TableView>,
        column: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidSelectColumnEvent", (tableView, column))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NumberOfCells(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NumberOfCells", ())?;
        Ok(__cordl_ret.into())
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
    pub fn SelectCellWithIdx(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectCellWithIdx", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        beatmapCharacteristicCollection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (beatmapCharacteristicCollection))?;
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
    pub fn add_didSelectCharacteristic(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::BeatmapCharacteristicSO>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectCharacteristic", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSelectCharacteristic(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::BeatmapCharacteristicSO>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectCharacteristic", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapCharacteristicsTableView")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapCharacteristicsTableView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapCharacteristicsTableView")]
impl AsRef<crate::HMUI::TableView_IDataSource>
for crate::GlobalNamespace::BeatmapCharacteristicsTableView {
    fn as_ref(&self) -> &crate::HMUI::TableView_IDataSource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapCharacteristicsTableView")]
impl AsMut<crate::HMUI::TableView_IDataSource>
for crate::GlobalNamespace::BeatmapCharacteristicsTableView {
    fn as_mut(&mut self) -> &mut crate::HMUI::TableView_IDataSource {
        unsafe { std::mem::transmute(self) }
    }
}
