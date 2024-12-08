#[cfg(feature = "HMUI+InputFieldDataBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct InputFieldDataBinder {
    __cordl_parent: crate::System::Object,
    pub _bindings: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Tuple_4<
            *mut crate::UnityEngine::UI::InputField,
            *mut IObservableChange,
            *mut crate::UnityEngine::Events::UnityAction_1<*mut crate::System::String>,
            *mut crate::System::Action,
        >,
    >,
}
#[cfg(feature = "HMUI+InputFieldDataBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::InputFieldDataBinder => "HMUI"
    ."InputFieldDataBinder"
);
#[cfg(feature = "HMUI+InputFieldDataBinder")]
impl std::ops::Deref for crate::HMUI::InputFieldDataBinder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+InputFieldDataBinder")]
impl std::ops::DerefMut for crate::HMUI::InputFieldDataBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+InputFieldDataBinder")]
impl crate::HMUI::InputFieldDataBinder {
    #[cfg(feature = "HMUI+InputFieldDataBinder+__c__DisplayClass2_0_2")]
    pub type __c__DisplayClass2_0_2<
        T0: quest_hook::libil2cpp::Type,
        T1: quest_hook::libil2cpp::Type,
    > = crate::HMUI::InputFieldDataBinder___c__DisplayClass2_0_2<T0, T1>;
    #[cfg(feature = "HMUI+InputFieldDataBinder+__c__3_1")]
    pub type __c__3_1<T: quest_hook::libil2cpp::Type> = crate::HMUI::InputFieldDataBinder___c__3_1<
        T,
    >;
    pub fn AddBindings<T0, T1>(
        &mut self,
        bindingData: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Tuple_4<
                *mut crate::UnityEngine::UI::InputField,
                T0,
                *mut crate::System::Func_2<*mut crate::System::String, T1>,
                *mut crate::System::Func_2<T1, *mut crate::System::String>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddBindings", (bindingData))?;
        Ok(__cordl_ret)
    }
    pub fn AddStringBindings<T>(
        &mut self,
        bindingData: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Tuple_2<*mut crate::UnityEngine::UI::InputField, T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddStringBindings", (bindingData))?;
        Ok(__cordl_ret)
    }
    pub fn ClearBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearBindings", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HMUI+InputFieldDataBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::InputFieldDataBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
