#[cfg(feature = "cordl_class_Assets+OVR+Scripts+FixRecord")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct FixRecord {
    __cordl_parent: crate::Assets::OVR::Scripts::Record,
    pub fixMethod: quest_hook::libil2cpp::Gc<crate::Assets::OVR::Scripts::FixMethodDelegate>,
    pub targetObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    pub buttonNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub editModeRequired: bool,
    pub complete: bool,
}
#[cfg(feature = "cordl_class_Assets+OVR+Scripts+FixRecord")]
unsafe impl quest_hook::libil2cpp::Type for crate::Assets::OVR::Scripts::FixRecord {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Assets.OVR.Scripts";
    const CLASS_NAME: &'static str = "FixRecord";
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
#[cfg(feature = "Assets+OVR+Scripts+FixRecord")]
impl std::ops::Deref for crate::Assets::OVR::Scripts::FixRecord {
    type Target = crate::Assets::OVR::Scripts::Record;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Assets+OVR+Scripts+FixRecord")]
impl std::ops::DerefMut for crate::Assets::OVR::Scripts::FixRecord {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Assets+OVR+Scripts+FixRecord")]
impl crate::Assets::OVR::Scripts::FixRecord {
    pub fn New(
        order: i32,
        cat: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fix: quest_hook::libil2cpp::Gc<crate::Assets::OVR::Scripts::FixMethodDelegate>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        editRequired: bool,
        buttons: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (order, cat, msg, fix, target, editRequired, buttons),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        order: i32,
        cat: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fix: quest_hook::libil2cpp::Gc<crate::Assets::OVR::Scripts::FixMethodDelegate>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        editRequired: bool,
        buttons: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::Assets::OVR::Scripts::FixMethodDelegate>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                        bool,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 7usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (order, cat, msg, fix, target, editRequired, buttons))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Assets+OVR+Scripts+FixRecord")]
impl quest_hook::libil2cpp::ObjectType for crate::Assets::OVR::Scripts::FixRecord {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
