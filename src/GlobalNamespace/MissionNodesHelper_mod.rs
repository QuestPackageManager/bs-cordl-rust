#[cfg(feature = "cordl_class_MissionNodesHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionNodesHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_MissionNodesHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::MissionNodesHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MissionNodesHelper";
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
#[cfg(feature = "MissionNodesHelper")]
impl std::ops::Deref for crate::GlobalNamespace::MissionNodesHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionNodesHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionNodesHelper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionNodesHelper")]
impl crate::GlobalNamespace::MissionNodesHelper {
    pub fn CycleDetection_MissionNode0(
        node: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>),
                        bool,
                        1usize,
                    >("CycleDetection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CycleDetection", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn CycleDetection_i32_Dictionary_2_1(
        node: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
        layer: i32,
        layers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
                i32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
                                i32,
                            >,
                        >,
                    ), bool, 3usize>("CycleDetection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CycleDetection",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (node, layer, layers))? };
        Ok(__cordl_ret.into())
    }
    pub fn FinalNodeIsFinal_HashSet_1_1(
        finalNode: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
        node: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
        visitedNodes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::HashSet_1<
                                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
                            >,
                        >,
                    ), bool, 3usize>("FinalNodeIsFinal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FinalNodeIsFinal",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (finalNode, node, visitedNodes))? };
        Ok(__cordl_ret.into())
    }
    pub fn FinalNodeIsFinal_MissionNode_MissionNode0(
        finalNode: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
        rootNode: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
                    ), bool, 2usize>("FinalNodeIsFinal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FinalNodeIsFinal",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (finalNode, rootNode))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllNodesFromRoot(
        root: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::HashSet_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::MissionNode,
                                >,
                            >,
                        >,
                        1usize,
                    >("GetAllNodesFromRoot")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAllNodesFromRoot", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (root))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn VisitAllTree(
        node: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
        visitedNodes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::HashSet_1<
                                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("VisitAllTree")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "VisitAllTree",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (node, visitedNodes))? };
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
#[cfg(feature = "cordl_class_MissionNodesHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MissionNodesHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
