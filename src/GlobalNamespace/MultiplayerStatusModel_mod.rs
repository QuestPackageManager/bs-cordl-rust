#[cfg(feature = "MultiplayerStatusModel")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerStatusModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _networkConfig: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::INetworkConfig,
    >,
    pub _client: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpClient>,
    pub _request: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::Task_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerStatusData>,
        >,
    >,
}
#[cfg(feature = "MultiplayerStatusModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerStatusModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerStatusModel";
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
#[cfg(feature = "MultiplayerStatusModel")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerStatusModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerStatusModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerStatusModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerStatusModel")]
impl crate::GlobalNamespace::MultiplayerStatusModel {
    pub const kRequestTimeoutSeconds: i32 = 60i32;
    #[cfg(feature = "MultiplayerStatusModel+MultiplayerStatusDataFB")]
    pub type MultiplayerStatusDataFB = crate::GlobalNamespace::MultiplayerStatusModel_MultiplayerStatusDataFB;
    pub fn GetMultiplayerStatusAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerStatusData>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Threading::CancellationToken),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MultiplayerStatusData,
                        >,
                    >,
                >,
                1usize,
            >("GetMultiplayerStatusAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetMultiplayerStatusAsync", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerStatusData>,
            >,
        > = unsafe { method.invoke_unchecked(self, (cancellationToken)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetMultiplayerStatusAsyncInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerStatusData>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MultiplayerStatusData,
                        >,
                    >,
                >,
                0usize,
            >("GetMultiplayerStatusAsyncInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetMultiplayerStatusAsyncInternal", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerStatusData>,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
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
    pub fn IsAvailabilityTaskValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsAvailabilityTaskValid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsAvailabilityTaskValid", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn StartRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("StartRequest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "StartRequest", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
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
}
#[cfg(feature = "MultiplayerStatusModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerStatusModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerStatusModel")]
impl AsRef<crate::GlobalNamespace::IMultiplayerStatusModel>
for crate::GlobalNamespace::MultiplayerStatusModel {
    fn as_ref(&self) -> &crate::GlobalNamespace::IMultiplayerStatusModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerStatusModel")]
impl AsMut<crate::GlobalNamespace::IMultiplayerStatusModel>
for crate::GlobalNamespace::MultiplayerStatusModel {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IMultiplayerStatusModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerStatusModel+MultiplayerStatusDataFB")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerStatusModel_MultiplayerStatusDataFB {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub data: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerStatusData>,
        >,
    >,
}
#[cfg(feature = "MultiplayerStatusModel+MultiplayerStatusDataFB")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerStatusModel_MultiplayerStatusDataFB {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerStatusModel/MultiplayerStatusDataFB";
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
#[cfg(feature = "MultiplayerStatusModel+MultiplayerStatusDataFB")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerStatusModel_MultiplayerStatusDataFB {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerStatusModel+MultiplayerStatusDataFB")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerStatusModel_MultiplayerStatusDataFB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerStatusModel+MultiplayerStatusDataFB")]
impl crate::GlobalNamespace::MultiplayerStatusModel_MultiplayerStatusDataFB {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "MultiplayerStatusModel+MultiplayerStatusDataFB")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerStatusModel_MultiplayerStatusDataFB {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
