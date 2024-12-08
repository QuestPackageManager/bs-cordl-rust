#[cfg(feature = "ISonyPSPremiumHelper+DisplayJoinPremiumDialogResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ISonyPSPremiumHelper_DisplayJoinPremiumDialogResult {
    Failed = 1i32,
    OK = 0i32,
}
#[cfg(feature = "ISonyPSPremiumHelper+DisplayJoinPremiumDialogResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ISonyPSPremiumHelper_DisplayJoinPremiumDialogResult => ""
    ."ISonyPSPremiumHelper/DisplayJoinPremiumDialogResult"
);
#[cfg(feature = "ISonyPSPremiumHelper+GetPremiumStatusResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ISonyPSPremiumHelper_GetPremiumStatusResult {
    Authorized = 0i32,
    Failed = 2i32,
    Unauthorized = 1i32,
}
#[cfg(feature = "ISonyPSPremiumHelper+GetPremiumStatusResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ISonyPSPremiumHelper_GetPremiumStatusResult => ""
    ."ISonyPSPremiumHelper/GetPremiumStatusResult"
);
#[cfg(feature = "ISonyPSPremiumHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct ISonyPSPremiumHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ISonyPSPremiumHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ISonyPSPremiumHelper => ""."ISonyPSPremiumHelper"
);
#[cfg(feature = "ISonyPSPremiumHelper")]
impl std::ops::Deref for ISonyPSPremiumHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ISonyPSPremiumHelper")]
impl std::ops::DerefMut for ISonyPSPremiumHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ISonyPSPremiumHelper")]
impl ISonyPSPremiumHelper {
    #[cfg(feature = "ISonyPSPremiumHelper+GetPremiumStatusResult")]
    pub type GetPremiumStatusResult = crate::GlobalNamespace::ISonyPSPremiumHelper_GetPremiumStatusResult;
    #[cfg(feature = "ISonyPSPremiumHelper+DisplayJoinPremiumDialogResult")]
    pub type DisplayJoinPremiumDialogResult = crate::GlobalNamespace::ISonyPSPremiumHelper_DisplayJoinPremiumDialogResult;
    pub fn DisplayJoinPremiumDialogAsync(
        &mut self,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::ISonyPSPremiumHelper_DisplayJoinPremiumDialogResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::ISonyPSPremiumHelper_DisplayJoinPremiumDialogResult,
        > = __cordl_object.invoke("DisplayJoinPremiumDialogAsync", (token))?;
        Ok(__cordl_ret)
    }
    pub fn GetPremiumStatusAsync(
        &mut self,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::ISonyPSPremiumHelper_GetPremiumStatusResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::ISonyPSPremiumHelper_GetPremiumStatusResult,
        > = __cordl_object.invoke("GetPremiumStatusAsync", (token))?;
        Ok(__cordl_ret)
    }
    pub fn NotifyPremiumFeature(
        &mut self,
        isSpectator: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyPremiumFeature", (isSpectator))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ISonyPSPremiumHelper")]
impl quest_hook::libil2cpp::ObjectType for ISonyPSPremiumHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
