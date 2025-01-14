#[cfg(feature = "GameServersListTableView")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServersListTableView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _tableView: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TableViewWithDetailCell,
    >,
    pub _gameServerListCellPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameServerListTableCell,
    >,
    pub _gameServerDetailCellPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameServerListDetailTableCell,
    >,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub joinButtonPressedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
        >,
    >,
    pub _isInitialized: bool,
    pub _gamesList: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
        >,
    >,
    pub _selectedServer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::INetworkPlayer,
    >,
}
#[cfg(feature = "GameServersListTableView")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameServersListTableView {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameServersListTableView";
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
#[cfg(feature = "GameServersListTableView")]
impl std::ops::Deref for crate::GlobalNamespace::GameServersListTableView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameServersListTableView")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameServersListTableView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameServersListTableView")]
impl crate::GlobalNamespace::GameServersListTableView {
    pub const kCellReuseIdentifier: &'static str = "Cell";
    pub const kDetailCellReuseIdentifier: &'static str = "DetailCell";
    pub fn CellForContent(
        &mut self,
        tableView: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TableViewWithDetailCell,
        >,
        idx: i32,
        detailOpened: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::TableCell>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::TableViewWithDetailCell,
                    >,
                    i32,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<crate::HMUI::TableCell>,
                3usize,
            >("CellForContent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CellForContent", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::TableCell> = unsafe {
            method.invoke_unchecked(self, (tableView, idx, detailOpened))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CellForDetail(
        &mut self,
        tableView: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TableViewWithDetailCell,
        >,
        contentIdx: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::TableCell>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::TableViewWithDetailCell,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<crate::HMUI::TableCell>,
                2usize,
            >("CellForDetail")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CellForDetail", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::TableCell> = unsafe {
            method.invoke_unchecked(self, (tableView, contentIdx))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CellSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("CellSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CellSize", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn HandleGameServerListDetailTableCellJoinServerButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleGameServerListDetailTableCellJoinServerButtonWasPressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self,
                    "HandleGameServerListDetailTableCellJoinServerButtonWasPressed",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleTableViewDidDeselectCellWithIdx(
        &mut self,
        arg1: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::TableViewWithDetailCell>,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::TableViewWithDetailCell,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleTableViewDidDeselectCellWithIdx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleTableViewDidDeselectCellWithIdx", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (arg1, arg2))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleTableViewDidSelectCellWithIdx(
        &mut self,
        tableView: quest_hook::libil2cpp::Gc<crate::HMUI::TableView>,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HMUI::TableView>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleTableViewDidSelectCellWithIdx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleTableViewDidSelectCellWithIdx", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tableView, id))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Init", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("NumberOfCells")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NumberOfCells", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDestroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnDestroy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        servers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
            >,
        >,
        clearSelection: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::INetworkPlayer,
                            >,
                        >,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetData", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (servers, clearSelection))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_joinButtonPressedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_joinButtonPressedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_joinButtonPressedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_joinButtonPressedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_joinButtonPressedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_joinButtonPressedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameServersListTableView")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameServersListTableView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameServersListTableView")]
impl AsRef<crate::GlobalNamespace::TableViewWithDetailCell_IDataSource>
for crate::GlobalNamespace::GameServersListTableView {
    fn as_ref(&self) -> &crate::GlobalNamespace::TableViewWithDetailCell_IDataSource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameServersListTableView")]
impl AsMut<crate::GlobalNamespace::TableViewWithDetailCell_IDataSource>
for crate::GlobalNamespace::GameServersListTableView {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::TableViewWithDetailCell_IDataSource {
        unsafe { std::mem::transmute(self) }
    }
}
