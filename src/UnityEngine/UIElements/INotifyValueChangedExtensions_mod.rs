#[cfg(feature = "UnityEngine+UIElements+INotifyValueChangedExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct INotifyValueChangedExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+INotifyValueChangedExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::INotifyValueChangedExtensions => "UnityEngine.UIElements"
    ."INotifyValueChangedExtensions"
);
#[cfg(feature = "UnityEngine+UIElements+INotifyValueChangedExtensions")]
impl std::ops::Deref for crate::UnityEngine::UIElements::INotifyValueChangedExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+INotifyValueChangedExtensions")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::INotifyValueChangedExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
                *mut crate::UnityEngine::UIElements::ChangeEvent_1<T>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterValueChangedCallback", (control, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterValueChangedCallback<T>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::INotifyValueChanged_1<T>,
        >,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallback_1<
                *mut crate::UnityEngine::UIElements::ChangeEvent_1<T>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnregisterValueChangedCallback", (control, callback))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+INotifyValueChangedExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::INotifyValueChangedExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
