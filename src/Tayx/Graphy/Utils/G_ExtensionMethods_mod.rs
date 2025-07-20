#[cfg(feature = "Tayx+Graphy+Utils+G_ExtensionMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct G_ExtensionMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Tayx+Graphy+Utils+G_ExtensionMethods")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Tayx::Graphy::Utils::G_ExtensionMethods {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Tayx.Graphy.Utils";
    const CLASS_NAME: &'static str = "G_ExtensionMethods";
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
#[cfg(feature = "Tayx+Graphy+Utils+G_ExtensionMethods")]
impl std::ops::Deref for crate::Tayx::Graphy::Utils::G_ExtensionMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Utils+G_ExtensionMethods")]
impl std::ops::DerefMut for crate::Tayx::Graphy::Utils::G_ExtensionMethods {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Utils+G_ExtensionMethods")]
impl crate::Tayx::Graphy::Utils::G_ExtensionMethods {
    pub fn SetAllActive_List_1__cordl_bool0(
        gameObjects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
        active: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                                >,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                            >,
                        >,
                        2usize,
                    >("SetAllActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetAllActive", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (gameObjects, active))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetAllActive_List_1__cordl_bool1(
        images: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
            >,
        >,
        active: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
                                >,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
                            >,
                        >,
                        2usize,
                    >("SetAllActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetAllActive", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (images, active))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetOneActive(
        images: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
            >,
        >,
        active: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
                                >,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
                            >,
                        >,
                        2usize,
                    >("SetOneActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOneActive", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (images, active))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Tayx+Graphy+Utils+G_ExtensionMethods")]
impl quest_hook::libil2cpp::ObjectType
for crate::Tayx::Graphy::Utils::G_ExtensionMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
