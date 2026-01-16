#[cfg(feature = "cordl_class_BeatmapEventDataProcessor_1")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEventDataProcessor_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "cordl_class_BeatmapEventDataProcessor_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::BeatmapEventDataProcessor_1<T>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapEventDataProcessor`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("", "BeatmapEventDataProcessor`1")
                .unwrap()
                .make_generic::<(T)>()
                .unwrap()
                .unwrap()
        })
    }
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
#[cfg(feature = "BeatmapEventDataProcessor_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::GlobalNamespace::BeatmapEventDataProcessor_1<T>
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataProcessor_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::GlobalNamespace::BeatmapEventDataProcessor_1<T>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataProcessor_1")]
impl<T: quest_hook::libil2cpp::Type> crate::GlobalNamespace::BeatmapEventDataProcessor_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessBeforeDeleteData(
        &mut self,
        nodeToDelete: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::LinkedListNode_1<
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "ProcessBeforeDeleteData"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessBeforeDeleteData",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nodeToDelete))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessBeforeDeleteEventDataCommon(
        nodeToDelete: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::LinkedListNode_1<
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "ProcessBeforeDeleteEventDataCommon"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessBeforeDeleteEventDataCommon",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (nodeToDelete))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessBeforeDeleteEventDataInternal(
        &mut self,
        nodeToDelete: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::LinkedListNode_1<
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "ProcessBeforeDeleteEventDataInternal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessBeforeDeleteEventDataInternal",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nodeToDelete))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessInsertedData(
        &mut self,
        insertedNode: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::LinkedListNode_1<
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "ProcessInsertedData"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessInsertedData",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (insertedNode))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessInsertedEventDataCommon(
        insertedNode: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::LinkedListNode_1<
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "ProcessInsertedEventDataCommon"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessInsertedEventDataCommon",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (insertedNode))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessInsertedEventDataInternal(
        &mut self,
        insertedNode: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::LinkedListNode_1<
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "ProcessInsertedEventDataInternal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessInsertedEventDataInternal",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (insertedNode))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
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
#[cfg(feature = "cordl_class_BeatmapEventDataProcessor_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::BeatmapEventDataProcessor_1<T>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapEventDataProcessor_1")]
impl<T: quest_hook::libil2cpp::Type>
    AsRef<
        crate::GlobalNamespace::ISortedListItemProcessor_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
        >,
    > for crate::GlobalNamespace::BeatmapEventDataProcessor_1<T>
{
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::ISortedListItemProcessor_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapEventDataProcessor_1")]
impl<T: quest_hook::libil2cpp::Type>
    AsMut<
        crate::GlobalNamespace::ISortedListItemProcessor_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
        >,
    > for crate::GlobalNamespace::BeatmapEventDataProcessor_1<T>
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ISortedListItemProcessor_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
