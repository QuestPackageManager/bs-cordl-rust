#[cfg(feature = "cordl_class_ResultsTableCell")]
#[repr(C)]
#[derive(Debug)]
pub struct ResultsTableCell {
    __cordl_parent: crate::GlobalNamespace::TableCellWithSeparator,
    pub _border: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _orderText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _nameText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _scoreText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _rankText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
}
#[cfg(feature = "cordl_class_ResultsTableCell")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::ResultsTableCell {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ResultsTableCell";
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
#[cfg(feature = "ResultsTableCell")]
impl std::ops::Deref for crate::GlobalNamespace::ResultsTableCell {
    type Target = crate::GlobalNamespace::TableCellWithSeparator;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ResultsTableCell")]
impl std::ops::DerefMut for crate::GlobalNamespace::ResultsTableCell {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ResultsTableCell")]
impl crate::GlobalNamespace::ResultsTableCell {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetData(
        &mut self,
        order: i32,
        connectedPlayer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetData",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (order, connectedPlayer, levelCompletionResults))?
        };
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
#[cfg(feature = "cordl_class_ResultsTableCell")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ResultsTableCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
