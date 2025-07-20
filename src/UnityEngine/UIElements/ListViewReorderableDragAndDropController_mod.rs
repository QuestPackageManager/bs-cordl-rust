#[cfg(feature = "UnityEngine+UIElements+ListViewReorderableDragAndDropController")]
#[repr(C)]
#[derive(Debug)]
pub struct ListViewReorderableDragAndDropController {
    __cordl_parent: crate::UnityEngine::UIElements::BaseReorderableDragAndDropController,
    pub m_ListView: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::BaseListView,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+ListViewReorderableDragAndDropController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::ListViewReorderableDragAndDropController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "ListViewReorderableDragAndDropController";
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
#[cfg(feature = "UnityEngine+UIElements+ListViewReorderableDragAndDropController")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::ListViewReorderableDragAndDropController {
    type Target = crate::UnityEngine::UIElements::BaseReorderableDragAndDropController;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListViewReorderableDragAndDropController")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::ListViewReorderableDragAndDropController {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListViewReorderableDragAndDropController")]
impl crate::UnityEngine::UIElements::ListViewReorderableDragAndDropController {
    pub fn HandleDragAndDrop(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IListDragAndDropArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::DragVisualMode> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::IListDragAndDropArgs,
                        >),
                        crate::UnityEngine::UIElements::DragVisualMode,
                        1usize,
                    >("HandleDragAndDrop")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleDragAndDrop", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::DragVisualMode = unsafe {
            method.invoke_unchecked(self, (args))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        view: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BaseListView>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (view))?;
        Ok(__cordl_object.into())
    }
    pub fn OnDrop(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IListDragAndDropArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::IListDragAndDropArgs,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnDrop")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "OnDrop",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (args))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        view: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BaseListView>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::BaseListView,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (view))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListViewReorderableDragAndDropController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ListViewReorderableDragAndDropController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
