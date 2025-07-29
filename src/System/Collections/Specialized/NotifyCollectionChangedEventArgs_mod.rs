#[cfg(
    feature = "cordl_class_System+Collections+Specialized+NotifyCollectionChangedEventArgs"
)]
#[repr(C)]
#[derive(Debug)]
pub struct NotifyCollectionChangedEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub _action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
    pub _newItems: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub _oldItems: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub _newStartingIndex: i32,
    pub _oldStartingIndex: i32,
}
#[cfg(
    feature = "cordl_class_System+Collections+Specialized+NotifyCollectionChangedEventArgs"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Collections::Specialized::NotifyCollectionChangedEventArgs {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Collections.Specialized";
    const CLASS_NAME: &'static str = "NotifyCollectionChangedEventArgs";
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
#[cfg(
    feature = "cordl_class_System+Collections+Specialized+NotifyCollectionChangedEventArgs"
)]
impl std::ops::Deref
for crate::System::Collections::Specialized::NotifyCollectionChangedEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "cordl_class_System+Collections+Specialized+NotifyCollectionChangedEventArgs"
)]
impl std::ops::DerefMut
for crate::System::Collections::Specialized::NotifyCollectionChangedEventArgs {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Specialized+NotifyCollectionChangedEventArgs")]
impl crate::System::Collections::Specialized::NotifyCollectionChangedEventArgs {
    pub fn InitializeAdd(
        &mut self,
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        newItems: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        newStartingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::Collections::Specialized::NotifyCollectionChangedAction,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InitializeAdd")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitializeAdd", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (action, newItems, newStartingIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitializeAddOrRemove(
        &mut self,
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        changedItems: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        startingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::Collections::Specialized::NotifyCollectionChangedAction,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InitializeAddOrRemove")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitializeAddOrRemove", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (action, changedItems, startingIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitializeMoveOrReplace(
        &mut self,
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        newItems: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        oldItems: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        startingIndex: i32,
        oldStartingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::Collections::Specialized::NotifyCollectionChangedAction,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("InitializeMoveOrReplace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitializeMoveOrReplace", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (action, newItems, oldItems, startingIndex, oldStartingIndex),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitializeRemove(
        &mut self,
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        oldItems: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        oldStartingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::Collections::Specialized::NotifyCollectionChangedAction,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InitializeRemove")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitializeRemove", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (action, oldItems, oldStartingIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_IList_IList_i32_3(
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        newItems: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        oldItems: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        startingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action, newItems, oldItems, startingIndex))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppObject_Il2CppObject_i32_2(
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        newItem: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        oldItem: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action, newItem, oldItem, index))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppObject_i32_1(
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        changedItem: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action, changedItem, index))?;
        Ok(__cordl_object.into())
    }
    pub fn New_NotifyCollectionChangedAction0(
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_IList_IList_i32_3(
        &mut self,
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        newItems: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        oldItems: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        startingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::Collections::Specialized::NotifyCollectionChangedAction,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (action, newItems, oldItems, startingIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject_Il2CppObject_i32_2(
        &mut self,
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        newItem: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        oldItem: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::Collections::Specialized::NotifyCollectionChangedAction,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (action, newItem, oldItem, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject_i32_1(
        &mut self,
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
        changedItem: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::Collections::Specialized::NotifyCollectionChangedAction,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (action, changedItem, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_NotifyCollectionChangedAction0(
        &mut self,
        action: crate::System::Collections::Specialized::NotifyCollectionChangedAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Collections::Specialized::NotifyCollectionChangedAction),
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
            cordl_method_info.invoke_unchecked(self, (action))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_System+Collections+Specialized+NotifyCollectionChangedEventArgs"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Specialized::NotifyCollectionChangedEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
