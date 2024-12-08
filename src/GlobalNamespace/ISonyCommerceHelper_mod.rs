#[cfg(feature = "ISonyCommerceHelper+AdditionalContentEntitlementsAsyncResult")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ISonyCommerceHelper_AdditionalContentEntitlementsAsyncResult {
    pub result: crate::GlobalNamespace::ISonyCommerceHelper_GetAdditionalContentEntitlementsResult,
    pub entitlementsLabels: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "ISonyCommerceHelper+AdditionalContentEntitlementsAsyncResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ISonyCommerceHelper_AdditionalContentEntitlementsAsyncResult => ""
    ."ISonyCommerceHelper/AdditionalContentEntitlementsAsyncResult"
);
#[cfg(feature = "ISonyCommerceHelper+AdditionalContentEntitlementsAsyncResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::ISonyCommerceHelper_AdditionalContentEntitlementsAsyncResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ISonyCommerceHelper+AdditionalContentEntitlementsAsyncResult")]
impl crate::GlobalNamespace::ISonyCommerceHelper_AdditionalContentEntitlementsAsyncResult {}
#[cfg(feature = "ISonyCommerceHelper+DisplayCategoryBrowseDialogResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ISonyCommerceHelper_DisplayCategoryBrowseDialogResult {
    Failed = 1i32,
    OK = 0i32,
}
#[cfg(feature = "ISonyCommerceHelper+DisplayCategoryBrowseDialogResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ISonyCommerceHelper_DisplayCategoryBrowseDialogResult => ""
    ."ISonyCommerceHelper/DisplayCategoryBrowseDialogResult"
);
#[cfg(feature = "ISonyCommerceHelper+DisplayProductBrowseDialogResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ISonyCommerceHelper_DisplayProductBrowseDialogResult {
    Failed = 1i32,
    OK = 0i32,
}
#[cfg(feature = "ISonyCommerceHelper+DisplayProductBrowseDialogResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ISonyCommerceHelper_DisplayProductBrowseDialogResult => ""
    ."ISonyCommerceHelper/DisplayProductBrowseDialogResult"
);
#[cfg(feature = "ISonyCommerceHelper+GetAdditionalContentEntitlementsResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ISonyCommerceHelper_GetAdditionalContentEntitlementsResult {
    Failed = 1i32,
    OK = 0i32,
}
#[cfg(feature = "ISonyCommerceHelper+GetAdditionalContentEntitlementsResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ISonyCommerceHelper_GetAdditionalContentEntitlementsResult => ""
    ."ISonyCommerceHelper/GetAdditionalContentEntitlementsResult"
);
#[cfg(feature = "ISonyCommerceHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct ISonyCommerceHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ISonyCommerceHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ISonyCommerceHelper => ""."ISonyCommerceHelper"
);
#[cfg(feature = "ISonyCommerceHelper")]
impl std::ops::Deref for ISonyCommerceHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ISonyCommerceHelper")]
impl std::ops::DerefMut for ISonyCommerceHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ISonyCommerceHelper")]
impl ISonyCommerceHelper {
    #[cfg(feature = "ISonyCommerceHelper+DisplayCategoryBrowseDialogResult")]
    pub type DisplayCategoryBrowseDialogResult = crate::GlobalNamespace::ISonyCommerceHelper_DisplayCategoryBrowseDialogResult;
    #[cfg(feature = "ISonyCommerceHelper+DisplayProductBrowseDialogResult")]
    pub type DisplayProductBrowseDialogResult = crate::GlobalNamespace::ISonyCommerceHelper_DisplayProductBrowseDialogResult;
    #[cfg(feature = "ISonyCommerceHelper+GetAdditionalContentEntitlementsResult")]
    pub type GetAdditionalContentEntitlementsResult = crate::GlobalNamespace::ISonyCommerceHelper_GetAdditionalContentEntitlementsResult;
    #[cfg(feature = "ISonyCommerceHelper+AdditionalContentEntitlementsAsyncResult")]
    pub type AdditionalContentEntitlementsAsyncResult = crate::GlobalNamespace::ISonyCommerceHelper_AdditionalContentEntitlementsAsyncResult;
    pub fn GetAdditionalContentEntitlementsAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::ISonyCommerceHelper_AdditionalContentEntitlementsAsyncResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::ISonyCommerceHelper_AdditionalContentEntitlementsAsyncResult,
        > = __cordl_object
            .invoke("GetAdditionalContentEntitlementsAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn DisplayProductBrowseDialogAsync(
        &mut self,
        productLabel: *mut crate::System::String,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::ISonyCommerceHelper_DisplayProductBrowseDialogResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::ISonyCommerceHelper_DisplayProductBrowseDialogResult,
        > = __cordl_object
            .invoke("DisplayProductBrowseDialogAsync", (productLabel, token))?;
        Ok(__cordl_ret)
    }
    pub fn DisplayCategoryBrowseDialogAsync(
        &mut self,
        categoryLabel: *mut crate::System::String,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::ISonyCommerceHelper_DisplayCategoryBrowseDialogResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::ISonyCommerceHelper_DisplayCategoryBrowseDialogResult,
        > = __cordl_object
            .invoke("DisplayCategoryBrowseDialogAsync", (categoryLabel, token))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ISonyCommerceHelper")]
impl quest_hook::libil2cpp::ObjectType for ISonyCommerceHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
