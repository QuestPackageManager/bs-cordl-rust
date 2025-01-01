#[cfg(feature = "HMUI+TableViewWithVariableSizedCells")]
#[repr(C)]
#[derive(Debug)]
pub struct TableViewWithVariableSizedCells {
    __cordl_parent: crate::HMUI::TableView,
    pub _totalHeight: f32,
    pub _cachedCellSizes: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _cachedCumulativeCellSizes: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
}
#[cfg(feature = "HMUI+TableViewWithVariableSizedCells")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::TableViewWithVariableSizedCells => "HMUI"
    ."TableViewWithVariableSizedCells"
);
#[cfg(feature = "HMUI+TableViewWithVariableSizedCells")]
impl std::ops::Deref for crate::HMUI::TableViewWithVariableSizedCells {
    type Target = crate::HMUI::TableView;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+TableViewWithVariableSizedCells")]
impl std::ops::DerefMut for crate::HMUI::TableViewWithVariableSizedCells {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+TableViewWithVariableSizedCells")]
impl crate::HMUI::TableViewWithVariableSizedCells {
    pub fn GetCellPosition(&mut self, idx: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetCellPosition", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCellSize(&mut self, idx: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetCellSize", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxVisibleIdx(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetMaxVisibleIdx", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMinVisibleIdx(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetMinVisibleIdx", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateCachedData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCachedData", ())?;
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
    pub fn get_contentSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_contentSize", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+TableViewWithVariableSizedCells")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::TableViewWithVariableSizedCells {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
