#[cfg(feature = "HMUI+ITableCellOwner")]
#[repr(C)]
#[derive(Debug)]
pub struct ITableCellOwner {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HMUI+ITableCellOwner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ITableCellOwner => "HMUI"
    ."ITableCellOwner"
);
#[cfg(feature = "HMUI+ITableCellOwner")]
impl std::ops::Deref for crate::HMUI::ITableCellOwner {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ITableCellOwner")]
impl std::ops::DerefMut for crate::HMUI::ITableCellOwner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ITableCellOwner")]
impl crate::HMUI::ITableCellOwner {
    pub fn get_selectionType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::HMUI::TableViewSelectionType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HMUI::TableViewSelectionType = __cordl_object
            .invoke("get_selectionType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_numberOfCells(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_numberOfCells", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_canSelectSelectedCell(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canSelectSelectedCell", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "HMUI+ITableCellOwner")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ITableCellOwner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
