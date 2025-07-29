#[cfg(feature = "cordl_class_ISonyCommerceHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct ISonyCommerceHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_ISonyCommerceHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::ISonyCommerceHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ISonyCommerceHelper";
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
#[cfg(feature = "ISonyCommerceHelper")]
impl std::ops::Deref for crate::GlobalNamespace::ISonyCommerceHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ISonyCommerceHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::ISonyCommerceHelper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ISonyCommerceHelper")]
impl crate::GlobalNamespace::ISonyCommerceHelper {
    #[cfg(feature = "ISonyCommerceHelper+AdditionalContentEntitlementsAsyncResult")]
    pub type AdditionalContentEntitlementsAsyncResult = crate::GlobalNamespace::ISonyCommerceHelper_AdditionalContentEntitlementsAsyncResult;
    #[cfg(feature = "ISonyCommerceHelper+DisplayCategoryBrowseDialogResult")]
    pub type DisplayCategoryBrowseDialogResult = crate::GlobalNamespace::ISonyCommerceHelper_DisplayCategoryBrowseDialogResult;
    #[cfg(feature = "ISonyCommerceHelper+DisplayProductBrowseDialogResult")]
    pub type DisplayProductBrowseDialogResult = crate::GlobalNamespace::ISonyCommerceHelper_DisplayProductBrowseDialogResult;
    #[cfg(feature = "ISonyCommerceHelper+GetAdditionalContentEntitlementsResult")]
    pub type GetAdditionalContentEntitlementsResult = crate::GlobalNamespace::ISonyCommerceHelper_GetAdditionalContentEntitlementsResult;
    pub fn DisplayCategoryBrowseDialogAsync(
        &mut self,
        categoryLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISonyCommerceHelper_DisplayCategoryBrowseDialogResult,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::ISonyCommerceHelper_DisplayCategoryBrowseDialogResult,
                        >,
                        2usize,
                    >("DisplayCategoryBrowseDialogAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DisplayCategoryBrowseDialogAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISonyCommerceHelper_DisplayCategoryBrowseDialogResult,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (categoryLabel, token))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisplayProductBrowseDialogAsync(
        &mut self,
        productLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISonyCommerceHelper_DisplayProductBrowseDialogResult,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::ISonyCommerceHelper_DisplayProductBrowseDialogResult,
                        >,
                        2usize,
                    >("DisplayProductBrowseDialogAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DisplayProductBrowseDialogAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISonyCommerceHelper_DisplayProductBrowseDialogResult,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (productLabel, token))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAdditionalContentEntitlementsAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISonyCommerceHelper_AdditionalContentEntitlementsAsyncResult,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Threading::CancellationToken),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::ISonyCommerceHelper_AdditionalContentEntitlementsAsyncResult,
                        >,
                        1usize,
                    >("GetAdditionalContentEntitlementsAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAdditionalContentEntitlementsAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISonyCommerceHelper_AdditionalContentEntitlementsAsyncResult,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (cancellationToken))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_ISonyCommerceHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ISonyCommerceHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_ISonyCommerceHelper+AdditionalContentEntitlementsAsyncResult"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ISonyCommerceHelper_AdditionalContentEntitlementsAsyncResult {
    pub result: crate::GlobalNamespace::ISonyCommerceHelper_GetAdditionalContentEntitlementsResult,
    pub entitlementsLabels: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
}
#[cfg(
    feature = "cordl_class_ISonyCommerceHelper+AdditionalContentEntitlementsAsyncResult"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ISonyCommerceHelper_AdditionalContentEntitlementsAsyncResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ISonyCommerceHelper/AdditionalContentEntitlementsAsyncResult";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_ISonyCommerceHelper+AdditionalContentEntitlementsAsyncResult"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::ISonyCommerceHelper_AdditionalContentEntitlementsAsyncResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_ISonyCommerceHelper+AdditionalContentEntitlementsAsyncResult"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::ISonyCommerceHelper_AdditionalContentEntitlementsAsyncResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_ISonyCommerceHelper+AdditionalContentEntitlementsAsyncResult"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::ISonyCommerceHelper_AdditionalContentEntitlementsAsyncResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "cordl_class_ISonyCommerceHelper+AdditionalContentEntitlementsAsyncResult"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::ISonyCommerceHelper_AdditionalContentEntitlementsAsyncResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_ISonyCommerceHelper+AdditionalContentEntitlementsAsyncResult"
)]
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
#[cfg(feature = "cordl_class_ISonyCommerceHelper+DisplayCategoryBrowseDialogResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ISonyCommerceHelper_DisplayCategoryBrowseDialogResult {
    #[default]
    Failed = 1i32,
    OK = 0i32,
}
#[cfg(feature = "cordl_class_ISonyCommerceHelper+DisplayCategoryBrowseDialogResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ISonyCommerceHelper_DisplayCategoryBrowseDialogResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ISonyCommerceHelper/DisplayCategoryBrowseDialogResult";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_ISonyCommerceHelper+DisplayCategoryBrowseDialogResult")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::ISonyCommerceHelper_DisplayCategoryBrowseDialogResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_ISonyCommerceHelper+DisplayCategoryBrowseDialogResult")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::ISonyCommerceHelper_DisplayCategoryBrowseDialogResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_ISonyCommerceHelper+DisplayCategoryBrowseDialogResult")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::ISonyCommerceHelper_DisplayCategoryBrowseDialogResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_ISonyCommerceHelper+DisplayCategoryBrowseDialogResult")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::ISonyCommerceHelper_DisplayCategoryBrowseDialogResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_ISonyCommerceHelper+DisplayProductBrowseDialogResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ISonyCommerceHelper_DisplayProductBrowseDialogResult {
    #[default]
    Failed = 1i32,
    OK = 0i32,
}
#[cfg(feature = "cordl_class_ISonyCommerceHelper+DisplayProductBrowseDialogResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ISonyCommerceHelper_DisplayProductBrowseDialogResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ISonyCommerceHelper/DisplayProductBrowseDialogResult";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_ISonyCommerceHelper+DisplayProductBrowseDialogResult")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::ISonyCommerceHelper_DisplayProductBrowseDialogResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_ISonyCommerceHelper+DisplayProductBrowseDialogResult")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::ISonyCommerceHelper_DisplayProductBrowseDialogResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_ISonyCommerceHelper+DisplayProductBrowseDialogResult")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::ISonyCommerceHelper_DisplayProductBrowseDialogResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_ISonyCommerceHelper+DisplayProductBrowseDialogResult")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::ISonyCommerceHelper_DisplayProductBrowseDialogResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_ISonyCommerceHelper+GetAdditionalContentEntitlementsResult"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ISonyCommerceHelper_GetAdditionalContentEntitlementsResult {
    #[default]
    Failed = 1i32,
    OK = 0i32,
}
#[cfg(
    feature = "cordl_class_ISonyCommerceHelper+GetAdditionalContentEntitlementsResult"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ISonyCommerceHelper_GetAdditionalContentEntitlementsResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ISonyCommerceHelper/GetAdditionalContentEntitlementsResult";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_ISonyCommerceHelper+GetAdditionalContentEntitlementsResult"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::ISonyCommerceHelper_GetAdditionalContentEntitlementsResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_ISonyCommerceHelper+GetAdditionalContentEntitlementsResult"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::ISonyCommerceHelper_GetAdditionalContentEntitlementsResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_ISonyCommerceHelper+GetAdditionalContentEntitlementsResult"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::ISonyCommerceHelper_GetAdditionalContentEntitlementsResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "cordl_class_ISonyCommerceHelper+GetAdditionalContentEntitlementsResult"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::ISonyCommerceHelper_GetAdditionalContentEntitlementsResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
