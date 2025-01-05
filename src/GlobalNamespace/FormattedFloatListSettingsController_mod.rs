#[cfg(feature = "FormattedFloatListSettingsController")]
#[repr(C)]
#[derive(Debug)]
pub struct FormattedFloatListSettingsController {
    __cordl_parent: crate::GlobalNamespace::ListSettingsController,
    pub _values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub _formattingString: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub valueType: crate::GlobalNamespace::FormattedFloatListSettingsController_ValueType,
    pub valueDidChangeEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::FormattedFloatListSettingsController,
            >,
            f32,
        >,
    >,
    pub _value: f32,
    pub _min: f32,
    pub _max: f32,
    pub _hasValue: bool,
}
#[cfg(feature = "FormattedFloatListSettingsController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::FormattedFloatListSettingsController => ""
    ."FormattedFloatListSettingsController"
);
#[cfg(feature = "FormattedFloatListSettingsController")]
impl std::ops::Deref for crate::GlobalNamespace::FormattedFloatListSettingsController {
    type Target = crate::GlobalNamespace::ListSettingsController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FormattedFloatListSettingsController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::FormattedFloatListSettingsController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FormattedFloatListSettingsController")]
impl crate::GlobalNamespace::FormattedFloatListSettingsController {
    #[cfg(feature = "FormattedFloatListSettingsController+ValueType")]
    pub type ValueType = crate::GlobalNamespace::FormattedFloatListSettingsController_ValueType;
    pub fn ApplyValue(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyValue", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInitValues(
        &mut self,
        idx: quest_hook::libil2cpp::ByRefMut<i32>,
        numberOfElements: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetInitValues", (idx, numberOfElements))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetValue(
        &mut self,
        value: f32,
        callCallback: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (value, callCallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn TextForValue(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("TextForValue", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn add_valueDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::FormattedFloatListSettingsController,
                >,
                f32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_valueDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_value(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_value", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_values(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = __cordl_object.invoke("get_values", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_valueDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::FormattedFloatListSettingsController,
                >,
                f32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_valueDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_values(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_values", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FormattedFloatListSettingsController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FormattedFloatListSettingsController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FormattedFloatListSettingsController+ValueType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormattedFloatListSettingsController_ValueType {
    #[default]
    InvertedNormalized = 2i32,
    Normal = 0i32,
    Normalized = 1i32,
}
#[cfg(feature = "FormattedFloatListSettingsController+ValueType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::FormattedFloatListSettingsController_ValueType => ""
    ."FormattedFloatListSettingsController/ValueType"
);
