#[cfg(feature = "cordl_class_UnityEngine+UIElements+INotifyValueChangedExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct INotifyValueChangedExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+INotifyValueChangedExtensions")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::INotifyValueChangedExtensions
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "INotifyValueChangedExtensions";
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
#[cfg(feature = "UnityEngine+UIElements+INotifyValueChangedExtensions")]
impl std::ops::Deref for crate::UnityEngine::UIElements::INotifyValueChangedExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+INotifyValueChangedExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::INotifyValueChangedExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+INotifyValueChangedExtensions")]
impl crate::UnityEngine::UIElements::INotifyValueChangedExtensions {
    pub fn RegisterValueChangedCallback<T>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::INotifyValueChanged_1<T>,
        >,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallback_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ChangeEvent_1<T>>,
            >,
        >,
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
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::INotifyValueChanged_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::EventCallback_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::UIElements::ChangeEvent_1<T>,
                                >,
                            >,
                        >,
                    ), bool, 2usize>("RegisterValueChangedCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RegisterValueChangedCallback",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (control, callback))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterValueChangedCallback<T>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::INotifyValueChanged_1<T>,
        >,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallback_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ChangeEvent_1<T>>,
            >,
        >,
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
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::INotifyValueChanged_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::EventCallback_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::UIElements::ChangeEvent_1<T>,
                                >,
                            >,
                        >,
                    ), bool, 2usize>("UnregisterValueChangedCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnregisterValueChangedCallback",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (control, callback))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+INotifyValueChangedExtensions")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::UIElements::INotifyValueChangedExtensions
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
