#[cfg(feature = "IPlatformAchievementsHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct IPlatformAchievementsHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IPlatformAchievementsHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IPlatformAchievementsHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IPlatformAchievementsHandler";
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
#[cfg(feature = "IPlatformAchievementsHandler")]
impl std::ops::Deref for crate::GlobalNamespace::IPlatformAchievementsHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IPlatformAchievementsHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::IPlatformAchievementsHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IPlatformAchievementsHandler")]
impl crate::GlobalNamespace::IPlatformAchievementsHandler {
    #[cfg(
        feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsCompletionHandler"
    )]
    pub type GetUnlockedAchievementsCompletionHandler = crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler;
    #[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsResult")]
    pub type GetUnlockedAchievementsResult = crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsResult;
    #[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementCompletionHandler")]
    pub type UnlockAchievementCompletionHandler = crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementCompletionHandler;
    #[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementResult")]
    pub type UnlockAchievementResult = crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementResult;
    pub fn GetUnlockedAchievements(
        &mut self,
        completionHandler: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HMAsyncRequest>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler,
                >),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HMAsyncRequest>,
                1usize,
            >("GetUnlockedAchievements")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetUnlockedAchievements", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::HMAsyncRequest,
        > = unsafe { method.invoke_unchecked(self, (completionHandler)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Initialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnlockAchievement(
        &mut self,
        achievementId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        completionHandler: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementCompletionHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HMAsyncRequest>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementCompletionHandler,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HMAsyncRequest>,
                2usize,
            >("UnlockAchievement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnlockAchievement", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::HMAsyncRequest,
        > = unsafe { method.invoke_unchecked(self, (achievementId, completionHandler)) };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IPlatformAchievementsHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IPlatformAchievementsHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsCompletionHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsCompletionHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IPlatformAchievementsHandler/GetUnlockedAchievementsCompletionHandler";
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
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsCompletionHandler")]
impl std::ops::Deref
for crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsCompletionHandler")]
impl std::ops::DerefMut
for crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsCompletionHandler")]
impl crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler {
    pub fn BeginInvoke(
        &mut self,
        result: crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsResult,
        unlockedAchievementsIds: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsResult,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                4usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BeginInvoke", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (result, unlockedAchievementsIds, callback, object),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EndInvoke", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        result: crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsResult,
        unlockedAchievementsIds: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsResult,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Invoke", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (result, unlockedAchievementsIds))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsCompletionHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IPlatformAchievementsHandler_GetUnlockedAchievementsResult {
    #[default]
    Failed = 1i32,
    OK = 0i32,
}
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IPlatformAchievementsHandler/GetUnlockedAchievementsResult";
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
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsResult")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsResult")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsResult")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsResult {
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
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsResult")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementCompletionHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct IPlatformAchievementsHandler_UnlockAchievementCompletionHandler {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementCompletionHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementCompletionHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IPlatformAchievementsHandler/UnlockAchievementCompletionHandler";
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
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementCompletionHandler")]
impl std::ops::Deref
for crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementCompletionHandler {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementCompletionHandler")]
impl std::ops::DerefMut
for crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementCompletionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementCompletionHandler")]
impl crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementCompletionHandler {
    pub fn BeginInvoke(
        &mut self,
        result: crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementResult,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementResult,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                3usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BeginInvoke", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (result, callback, object))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EndInvoke", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        result: crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementResult),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Invoke", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementCompletionHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementCompletionHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IPlatformAchievementsHandler_UnlockAchievementResult {
    #[default]
    Failed = 1i32,
    OK = 0i32,
}
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IPlatformAchievementsHandler/UnlockAchievementResult";
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
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementResult")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementResult")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementResult")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementResult {
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
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementResult")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
