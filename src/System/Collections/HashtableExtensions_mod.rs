#[cfg(feature = "cordl_class_System+Collections+HashtableExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct HashtableExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+Collections+HashtableExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Collections::HashtableExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Collections";
    const CLASS_NAME: &'static str = "HashtableExtensions";
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
#[cfg(feature = "System+Collections+HashtableExtensions")]
impl std::ops::Deref for crate::System::Collections::HashtableExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+HashtableExtensions")]
impl std::ops::DerefMut for crate::System::Collections::HashtableExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+HashtableExtensions")]
impl crate::System::Collections::HashtableExtensions {
    pub fn TryGetValue<T>(
        table: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::ByRefMut<T>,
                    ), bool, 3usize>("TryGetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (table, key, value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Collections+HashtableExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Collections::HashtableExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
