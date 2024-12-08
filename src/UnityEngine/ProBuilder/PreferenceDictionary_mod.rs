#[cfg(feature = "UnityEngine+ProBuilder+PreferenceDictionary")]
#[repr(C)]
#[derive(Debug)]
pub struct PreferenceDictionary {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub m_Bool: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        bool,
    >,
    pub m_Int: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        i32,
    >,
    pub m_Float: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        f32,
    >,
    pub m_String: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::System::String,
    >,
    pub m_Color: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        crate::UnityEngine::Color,
    >,
    pub m_Material: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::UnityEngine::Material,
    >,
    pub m_Bool_keys: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub m_Int_keys: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub m_Float_keys: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub m_String_keys: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub m_Color_keys: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub m_Material_keys: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub m_Bool_values: *mut crate::System::Collections::Generic::List_1<bool>,
    pub m_Int_values: *mut crate::System::Collections::Generic::List_1<i32>,
    pub m_Float_values: *mut crate::System::Collections::Generic::List_1<f32>,
    pub m_String_values: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub m_Color_values: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Color,
    >,
    pub m_Material_values: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Material,
    >,
}
#[cfg(feature = "UnityEngine+ProBuilder+PreferenceDictionary")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::PreferenceDictionary =>
    "UnityEngine.ProBuilder"."PreferenceDictionary"
);
#[cfg(feature = "UnityEngine+ProBuilder+PreferenceDictionary")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::PreferenceDictionary {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+PreferenceDictionary")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::PreferenceDictionary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+PreferenceDictionary")]
impl crate::UnityEngine::ProBuilder::PreferenceDictionary {
    pub fn DeleteKey(
        &mut self,
        key: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn Get<T>(
        &mut self,
        key: *mut crate::System::String,
        fallback: T,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("Get", (key, fallback))?;
        Ok(__cordl_ret)
    }
    pub fn GetBool(
        &mut self,
        key: *mut crate::System::String,
        fallback: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetBool", (key, fallback))?;
        Ok(__cordl_ret)
    }
    pub fn GetString(
        &mut self,
        key: *mut crate::System::String,
        fallback: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetString", (key, fallback))?;
        Ok(__cordl_ret)
    }
    pub fn GetMaterial(
        &mut self,
        key: *mut crate::System::String,
        fallback: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("GetMaterial", (key, fallback))?;
        Ok(__cordl_ret)
    }
    pub fn SetString(
        &mut self,
        key: *mut crate::System::String,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetString", (key, value))?;
        Ok(__cordl_ret)
    }
    pub fn GetBoolDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            bool,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            bool,
        > = __cordl_object.invoke("GetBoolDictionary", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetColorDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            crate::UnityEngine::Color,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            crate::UnityEngine::Color,
        > = __cordl_object.invoke("GetColorDictionary", ())?;
        Ok(__cordl_ret)
    }
    pub fn Set<T>(
        &mut self,
        key: *mut crate::System::String,
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (key, value))?;
        Ok(__cordl_ret)
    }
    pub fn GetFloatDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            f32,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            f32,
        > = __cordl_object.invoke("GetFloatDictionary", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetFloat(
        &mut self,
        key: *mut crate::System::String,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloat", (key, value))?;
        Ok(__cordl_ret)
    }
    pub fn GetStringDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        > = __cordl_object.invoke("GetStringDictionary", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetDefaultValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDefaultValues", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasKey_String0(
        &mut self,
        key: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn HasKey_String1<T>(
        &mut self,
        key: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn GetColor(
        &mut self,
        key: *mut crate::System::String,
        fallback: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("GetColor", (key, fallback))?;
        Ok(__cordl_ret)
    }
    pub fn SetInt(
        &mut self,
        key: *mut crate::System::String,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInt", (key, value))?;
        Ok(__cordl_ret)
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
    pub fn OnAfterDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAfterDeserialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetColor(
        &mut self,
        key: *mut crate::System::String,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColor", (key, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetMaterial(
        &mut self,
        key: *mut crate::System::String,
        value: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMaterial", (key, value))?;
        Ok(__cordl_ret)
    }
    pub fn OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeforeSerialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetInt(
        &mut self,
        key: *mut crate::System::String,
        fallback: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetInt", (key, fallback))?;
        Ok(__cordl_ret)
    }
    pub fn GetFloat(
        &mut self,
        key: *mut crate::System::String,
        fallback: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetFloat", (key, fallback))?;
        Ok(__cordl_ret)
    }
    pub fn SetBool(
        &mut self,
        key: *mut crate::System::String,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBool", (key, value))?;
        Ok(__cordl_ret)
    }
    pub fn GetIntDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            i32,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            i32,
        > = __cordl_object.invoke("GetIntDictionary", ())?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMaterialDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::UnityEngine::Material,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::UnityEngine::Material,
        > = __cordl_object.invoke("GetMaterialDictionary", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+PreferenceDictionary")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::PreferenceDictionary {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
