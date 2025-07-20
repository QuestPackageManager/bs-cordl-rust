#[cfg(feature = "HMUI+ITableCellOwner")]
#[repr(C)]
#[derive(Debug)]
pub struct ITableCellOwner {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HMUI+ITableCellOwner")]
unsafe impl quest_hook::libil2cpp::Type for crate::HMUI::ITableCellOwner {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HMUI";
    const CLASS_NAME: &'static str = "ITableCellOwner";
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_canSelectSelectedCell(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::ITableCellOwner as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_canSelectSelectedCell")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::ITableCellOwner as quest_hook::libil2cpp::Type >
                    ::class(), "get_canSelectSelectedCell", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_numberOfCells(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::ITableCellOwner as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_numberOfCells")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::ITableCellOwner as quest_hook::libil2cpp::Type >
                    ::class(), "get_numberOfCells", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_selectionType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::HMUI::TableViewSelectionType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::ITableCellOwner as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::HMUI::TableViewSelectionType,
                0usize,
            >("get_selectionType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::ITableCellOwner as quest_hook::libil2cpp::Type >
                    ::class(), "get_selectionType", 0usize
                )
            });
        let __cordl_ret: crate::HMUI::TableViewSelectionType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
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
