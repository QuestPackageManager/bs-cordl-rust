#[cfg(feature = "UnityEngine+ProBuilder+PreferenceDictionary")]
#[repr(C)]
#[derive(Debug)]
pub struct PreferenceDictionary {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub m_Bool: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        bool,
    >,
    pub m_Int: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        i32,
    >,
    pub m_Float: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        f32,
    >,
    pub m_String: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_Color: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        crate::UnityEngine::Color,
    >,
    pub m_Material: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::UnityEngine::Material,
    >,
    pub m_Bool_keys: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_Int_keys: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_Float_keys: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_String_keys: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_Color_keys: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_Material_keys: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_Bool_values: *mut crate::System::Collections::Generic::List_1<bool>,
    pub m_Int_values: *mut crate::System::Collections::Generic::List_1<i32>,
    pub m_Float_values: *mut crate::System::Collections::Generic::List_1<f32>,
    pub m_String_values: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppString,
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
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteKey(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteKey", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn Get<T>(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn GetBool(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fallback: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetBool", (key, fallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoolDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                bool,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                bool,
            >,
        > = __cordl_object.invoke("GetBoolDictionary", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetColor(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fallback: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("GetColor", (key, fallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetColorDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                crate::UnityEngine::Color,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                crate::UnityEngine::Color,
            >,
        > = __cordl_object.invoke("GetColorDictionary", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFloat(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fallback: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetFloat", (key, fallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFloatDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                f32,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                f32,
            >,
        > = __cordl_object.invoke("GetFloatDictionary", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInt(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fallback: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetInt", (key, fallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIntDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                i32,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                i32,
            >,
        > = __cordl_object.invoke("GetIntDictionary", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaterial(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fallback: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("GetMaterial", (key, fallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaterialDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::UnityEngine::Material,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::UnityEngine::Material,
            >,
        > = __cordl_object.invoke("GetMaterialDictionary", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetString(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fallback: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetString", (key, fallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStringDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("GetStringDictionary", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasKey_Il2CppString0(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasKey", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasKey_Il2CppString1<T>(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasKey", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnAfterDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAfterDeserialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeforeSerialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Set<T>(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn SetBool(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBool", (key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColor(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColor", (key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDefaultValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDefaultValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFloat(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloat", (key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInt(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInt", (key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMaterial(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMaterial", (key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetString(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetString", (key, value))?;
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
#[cfg(feature = "UnityEngine+ProBuilder+PreferenceDictionary")]
impl AsRef<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::UnityEngine::ProBuilder::PreferenceDictionary {
    fn as_ref(&self) -> &crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+PreferenceDictionary")]
impl AsMut<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::UnityEngine::ProBuilder::PreferenceDictionary {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+PreferenceDictionary")]
impl AsRef<crate::UnityEngine::ProBuilder::IHasDefault>
for crate::UnityEngine::ProBuilder::PreferenceDictionary {
    fn as_ref(&self) -> &crate::UnityEngine::ProBuilder::IHasDefault {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+PreferenceDictionary")]
impl AsMut<crate::UnityEngine::ProBuilder::IHasDefault>
for crate::UnityEngine::ProBuilder::PreferenceDictionary {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::ProBuilder::IHasDefault {
        unsafe { std::mem::transmute(self) }
    }
}
