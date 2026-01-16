#[cfg(feature = "cordl_class_UnityEngine+UIElements+BindingUpdater")]
#[repr(C)]
#[derive(Debug)]
pub struct BindingUpdater {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+BindingUpdater")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::BindingUpdater {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "BindingUpdater";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+UIElements+BindingUpdater")]
impl std::ops::Deref for crate::UnityEngine::UIElements::BindingUpdater {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BindingUpdater")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::BindingUpdater {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BindingUpdater")]
impl crate::UnityEngine::UIElements::BindingUpdater {
    #[cfg(feature = "UnityEngine+UIElements+BindingUpdater+CastDataSourceVisitor")]
    pub type CastDataSourceVisitor =
        crate::UnityEngine::UIElements::BindingUpdater_CastDataSourceVisitor;
    #[cfg(feature = "UnityEngine+UIElements+BindingUpdater+UIPathVisitor")]
    pub type UIPathVisitor = crate::UnityEngine::UIElements::BindingUpdater_UIPathVisitor;
    pub fn GetExtractValueErrorString(
        returnCode: crate::Unity::Properties::VisitReturnCode,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        path: quest_hook::libil2cpp::ByRefMut<crate::Unity::Properties::PropertyPath>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Properties::VisitReturnCode,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Properties::PropertyPath>,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, 3usize>(
                        "GetExtractValueErrorString",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetExtractValueErrorString",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (returnCode, target, path))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRootDataSourceError(
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("GetRootDataSourceError")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRootDataSourceError", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (target))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetVisitationErrorString(
        returnCode: crate::Unity::Properties::VisitReturnCode,
        context: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::BindingContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Properties::VisitReturnCode,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::BindingContext,
                        >,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, 2usize>(
                        "GetVisitationErrorString",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetVisitationErrorString",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (returnCode, context))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ShouldProcessBindingAtStage_Binding0(
        &mut self,
        bindingObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Binding>,
        stage: crate::UnityEngine::UIElements::BindingUpdateStage,
        versionChanged: bool,
        dirty: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Binding>,
                        crate::UnityEngine::UIElements::BindingUpdateStage,
                        bool,
                        bool,
                    ), bool, 4usize>("ShouldProcessBindingAtStage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ShouldProcessBindingAtStage",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (bindingObject, stage, versionChanged, dirty))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldProcessBindingAtStage_CustomBinding2(
        &mut self,
        customBinding: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::CustomBinding>,
        stage: crate::UnityEngine::UIElements::BindingUpdateStage,
        versionChanged: bool,
        dirty: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::CustomBinding>,
                        crate::UnityEngine::UIElements::BindingUpdateStage,
                        bool,
                        bool,
                    ), bool, 4usize>("ShouldProcessBindingAtStage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ShouldProcessBindingAtStage",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (customBinding, stage, versionChanged, dirty))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldProcessBindingAtStage_DataBinding1(
        dataBinding: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
        stage: crate::UnityEngine::UIElements::BindingUpdateStage,
        versionChanged: bool,
        dirty: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
                        crate::UnityEngine::UIElements::BindingUpdateStage,
                        bool,
                        bool,
                    ), bool, 4usize>("ShouldProcessBindingAtStage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ShouldProcessBindingAtStage",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (dataBinding, stage, versionChanged, dirty))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryUpdateUIWithNonContainer(
        context: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::BindingContext>,
        binding: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BindingResult> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::BindingContext,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), crate::UnityEngine::UIElements::BindingResult, 3usize>(
                        "TryUpdateUIWithNonContainer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryUpdateUIWithNonContainer",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BindingResult =
            unsafe { cordl_method_info.invoke_unchecked((), (context, binding, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateDataSource_CustomBinding1(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::BindingContext>,
        customBinding: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::CustomBinding>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BindingResult> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::BindingContext,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::CustomBinding>,
                    ), crate::UnityEngine::UIElements::BindingResult, 2usize>(
                        "UpdateDataSource"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateDataSource",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BindingResult =
            unsafe { cordl_method_info.invoke_unchecked(self, (context, customBinding))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateDataSource_DataBinding0(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::BindingContext>,
        dataBinding: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BindingResult> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::BindingContext,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
                    ), crate::UnityEngine::UIElements::BindingResult, 2usize>(
                        "UpdateDataSource"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateDataSource",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BindingResult =
            unsafe { cordl_method_info.invoke_unchecked(self, (context, dataBinding))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateSource(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::BindingContext>,
        bindingObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Binding>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BindingResult> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::BindingContext,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Binding>,
                    ), crate::UnityEngine::UIElements::BindingResult, 2usize>(
                        "UpdateSource"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateSource",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BindingResult =
            unsafe { cordl_method_info.invoke_unchecked(self, (context, bindingObject))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateUI_Binding0(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::BindingContext>,
        bindingObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Binding>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BindingResult> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::BindingContext,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Binding>,
                    ), crate::UnityEngine::UIElements::BindingResult, 2usize>(
                        "UpdateUI"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateUI",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BindingResult =
            unsafe { cordl_method_info.invoke_unchecked(self, (context, bindingObject))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateUI_CustomBinding2(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::BindingContext>,
        customBinding: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::CustomBinding>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BindingResult> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::BindingContext,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::CustomBinding>,
                    ), crate::UnityEngine::UIElements::BindingResult, 2usize>(
                        "UpdateUI"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateUI",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BindingResult =
            unsafe { cordl_method_info.invoke_unchecked(self, (context, customBinding))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateUI_DataBinding1(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::BindingContext>,
        dataBinding: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BindingResult> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::BindingContext,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
                    ), crate::UnityEngine::UIElements::BindingResult, 2usize>(
                        "UpdateUI"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateUI",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BindingResult =
            unsafe { cordl_method_info.invoke_unchecked(self, (context, dataBinding))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitAtPath<TContainer>(
        dataBinding: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
        direction: crate::UnityEngine::UIElements::BindingUpdateStage,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
        path: quest_hook::libil2cpp::ByRefMut<crate::Unity::Properties::PropertyPath>,
        context: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::BindingContext>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_4<
            bool,
            crate::Unity::Properties::VisitReturnCode,
            crate::Unity::Properties::VisitReturnCode,
            crate::UnityEngine::UIElements::BindingResult,
        >,
    >
    where
        TContainer: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
                        crate::UnityEngine::UIElements::BindingUpdateStage,
                        quest_hook::libil2cpp::ByRefMut<TContainer>,
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Properties::PropertyPath>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::BindingContext,
                        >,
                    ), crate::System::ValueTuple_4<
                        bool,
                        crate::Unity::Properties::VisitReturnCode,
                        crate::Unity::Properties::VisitReturnCode,
                        crate::UnityEngine::UIElements::BindingResult,
                    >, 5usize>("VisitAtPath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "VisitAtPath",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ValueTuple_4<
            bool,
            crate::Unity::Properties::VisitReturnCode,
            crate::Unity::Properties::VisitReturnCode,
            crate::UnityEngine::UIElements::BindingResult,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (dataBinding, direction, container, path, context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VisitRoot(
        dataBinding: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
        container: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        context: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::BindingContext>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_3<
            bool,
            crate::Unity::Properties::VisitReturnCode,
            crate::UnityEngine::UIElements::BindingResult,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::BindingContext,
                        >,
                    ), crate::System::ValueTuple_3<
                        bool,
                        crate::Unity::Properties::VisitReturnCode,
                        crate::UnityEngine::UIElements::BindingResult,
                    >, 3usize>("VisitRoot")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "VisitRoot",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ValueTuple_3<
            bool,
            crate::Unity::Properties::VisitReturnCode,
            crate::UnityEngine::UIElements::BindingResult,
        > = unsafe { cordl_method_info.invoke_unchecked((), (dataBinding, container, context))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+BindingUpdater")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::BindingUpdater {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+BindingUpdater+CastDataSourceVisitor")]
#[repr(C)]
#[derive(Debug)]
pub struct BindingUpdater_CastDataSourceVisitor {
    __cordl_parent: crate::Unity::Properties::ConcreteTypeVisitor,
    pub _Binding_k__BackingField:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
    pub _bindingContext_k__BackingField: crate::UnityEngine::UIElements::BindingContext,
    pub _result_k__BackingField: crate::UnityEngine::UIElements::BindingResult,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+BindingUpdater+CastDataSourceVisitor")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::BindingUpdater_CastDataSourceVisitor
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "BindingUpdater/CastDataSourceVisitor";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+UIElements+BindingUpdater+CastDataSourceVisitor")]
impl std::ops::Deref for crate::UnityEngine::UIElements::BindingUpdater_CastDataSourceVisitor {
    type Target = crate::Unity::Properties::ConcreteTypeVisitor;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BindingUpdater+CastDataSourceVisitor")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::BindingUpdater_CastDataSourceVisitor {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BindingUpdater+CastDataSourceVisitor")]
impl crate::UnityEngine::UIElements::BindingUpdater_CastDataSourceVisitor {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Reset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitContainer<TContainer>(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<TContainer>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("VisitContainer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitContainer", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (container))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Binding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::DataBinding,
                        >,
                        0usize,
                    >("get_Binding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Binding", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_bindingContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BindingContext> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::BindingContext, 0usize>(
                        "get_bindingContext",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_bindingContext",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BindingContext =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_result(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BindingResult> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::BindingResult, 0usize>(
                        "get_result",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_result",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BindingResult =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Binding(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::DataBinding,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_Binding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_Binding", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_bindingContext(
        &mut self,
        value: crate::UnityEngine::UIElements::BindingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::BindingContext),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_bindingContext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_bindingContext", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_result(
        &mut self,
        value: crate::UnityEngine::UIElements::BindingResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::BindingResult),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_result")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_result", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+BindingUpdater+CastDataSourceVisitor")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::UIElements::BindingUpdater_CastDataSourceVisitor
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+BindingUpdater+UIPathVisitor")]
#[repr(C)]
#[derive(Debug)]
pub struct BindingUpdater_UIPathVisitor {
    __cordl_parent: crate::Unity::Properties::PathVisitor,
    pub _binding_k__BackingField:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
    pub _direction_k__BackingField: crate::UnityEngine::UIElements::BindingUpdateStage,
    pub _bindingContext_k__BackingField: crate::UnityEngine::UIElements::BindingContext,
    pub _result_k__BackingField: crate::UnityEngine::UIElements::BindingResult,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+BindingUpdater+UIPathVisitor")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::BindingUpdater_UIPathVisitor
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "BindingUpdater/UIPathVisitor";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+UIElements+BindingUpdater+UIPathVisitor")]
impl std::ops::Deref for crate::UnityEngine::UIElements::BindingUpdater_UIPathVisitor {
    type Target = crate::Unity::Properties::PathVisitor;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BindingUpdater+UIPathVisitor")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::BindingUpdater_UIPathVisitor {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BindingUpdater+UIPathVisitor")]
impl crate::UnityEngine::UIElements::BindingUpdater_UIPathVisitor {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Reset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitPath<TContainer, TValue>(
        &mut self,
        property: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::Property_2<TContainer, TValue>,
        >,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
        value: quest_hook::libil2cpp::ByRefMut<TValue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::Unity::Properties::Property_2<TContainer, TValue>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<TContainer>,
                        quest_hook::libil2cpp::ByRefMut<TValue>,
                    ), quest_hook::libil2cpp::Void, 3usize>("VisitPath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "VisitPath",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (property, container, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_binding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::DataBinding,
                        >,
                        0usize,
                    >("get_binding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_binding", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_bindingContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BindingContext> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::BindingContext, 0usize>(
                        "get_bindingContext",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_bindingContext",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BindingContext =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_direction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BindingUpdateStage> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::BindingUpdateStage, 0usize>(
                        "get_direction",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_direction",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BindingUpdateStage =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_result(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BindingResult> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::BindingResult, 0usize>(
                        "get_result",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_result",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BindingResult =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_binding(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DataBinding>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::DataBinding,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_binding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_binding", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_bindingContext(
        &mut self,
        value: crate::UnityEngine::UIElements::BindingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::BindingContext),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_bindingContext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_bindingContext", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_direction(
        &mut self,
        value: crate::UnityEngine::UIElements::BindingUpdateStage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::BindingUpdateStage),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_direction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_direction", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_result(
        &mut self,
        value: crate::UnityEngine::UIElements::BindingResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::BindingResult),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_result")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_result", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+BindingUpdater+UIPathVisitor")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::UIElements::BindingUpdater_UIPathVisitor
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
