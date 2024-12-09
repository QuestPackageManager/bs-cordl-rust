#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_PluginStorage {
    __cordl_parent: crate::System::Object,
    pub _dataMap: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::HoudiniEngineUnity::HEU_PluginStorage_StoreData,
    >,
    pub _envPathMap: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::System::String,
    >,
    pub _requiresSave: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_PluginStorage =>
    "HoudiniEngineUnity"."HEU_PluginStorage"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_PluginStorage {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_PluginStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage")]
impl crate::HoudiniEngineUnity::HEU_PluginStorage {
    pub const PluginSettingsLine1: &'static str = "Houdini Engine for Unity Plugin Settings";
    pub const PluginSettingsLine2: &'static str = "Version=";
    pub const PluginSettingsVersion: &'static str = "1.0";
    #[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage+DataType")]
    pub type DataType = crate::HoudiniEngineUnity::HEU_PluginStorage_DataType;
    #[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage+StoreData")]
    pub type StoreData = crate::HoudiniEngineUnity::HEU_PluginStorage_StoreData;
    #[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage+StoreDataArray_1")]
    pub type StoreDataArray_1<T: quest_hook::libil2cpp::Type> = crate::HoudiniEngineUnity::HEU_PluginStorage_StoreDataArray_1<
        T,
    >;
    pub fn ConvertEnvKeyedPathToReal(
        &mut self,
        inPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ConvertEnvKeyedPathToReal", (inPath))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertRealPathToEnvKeyedPath(
        &mut self,
        inPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ConvertRealPathToEnvKeyedPath", (inPath))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnvironmentPathMap(
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
        > = __cordl_object.invoke("GetEnvironmentPathMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetJSONArray<T>(
        &mut self,
        jsonArray: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<T> = __cordl_object
            .invoke("GetJSONArray", (jsonArray))?;
        Ok(__cordl_ret)
    }
    pub fn Get_String4(
        &mut self,
        key: *mut crate::System::String,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
        defaultValue: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Get", (key, value, defaultValue))?;
        Ok(__cordl_ret)
    }
    pub fn Get__cordl_bool0(
        &mut self,
        key: *mut crate::System::String,
        value: quest_hook::libil2cpp::ByRefMut<bool>,
        defaultValue: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Get", (key, value, defaultValue))?;
        Ok(__cordl_ret)
    }
    pub fn Get__cordl_char5(
        &mut self,
        key: *mut crate::System::String,
        values: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
        >,
        delimiter: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Get", (key, values, delimiter))?;
        Ok(__cordl_ret)
    }
    pub fn Get_f32_3(
        &mut self,
        key: *mut crate::System::String,
        value: quest_hook::libil2cpp::ByRefMut<f32>,
        defaultValue: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Get", (key, value, defaultValue))?;
        Ok(__cordl_ret)
    }
    pub fn Get_i32_1(
        &mut self,
        key: *mut crate::System::String,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
        defaultValue: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Get", (key, value, defaultValue))?;
        Ok(__cordl_ret)
    }
    pub fn Get_i64_2(
        &mut self,
        key: *mut crate::System::String,
        value: quest_hook::libil2cpp::ByRefMut<i64>,
        defaultValue: i64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Get", (key, value, defaultValue))?;
        Ok(__cordl_ret)
    }
    pub fn LoadAssetEnvironmentPaths(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadAssetEnvironmentPaths", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadPluginData(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("LoadPluginData", ())?;
        Ok(__cordl_ret)
    }
    pub fn MarkDirtyForSave(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkDirtyForSave", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ReadFromEditorPrefs(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadFromEditorPrefs", ())?;
        Ok(__cordl_ret)
    }
    pub fn SavePluginData(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SavePluginData", ())?;
        Ok(__cordl_ret)
    }
    pub fn Set_List_1__cordl_char5(
        &mut self,
        key: *mut crate::System::String,
        values: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
        delimiter: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (key, values, delimiter))?;
        Ok(__cordl_ret)
    }
    pub fn Set_String4(
        &mut self,
        key: *mut crate::System::String,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (key, value))?;
        Ok(__cordl_ret)
    }
    pub fn Set__cordl_bool0(
        &mut self,
        key: *mut crate::System::String,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (key, value))?;
        Ok(__cordl_ret)
    }
    pub fn Set_f32_3(
        &mut self,
        key: *mut crate::System::String,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (key, value))?;
        Ok(__cordl_ret)
    }
    pub fn Set_i32_1(
        &mut self,
        key: *mut crate::System::String,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (key, value))?;
        Ok(__cordl_ret)
    }
    pub fn Set_i64_2(
        &mut self,
        key: *mut crate::System::String,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (key, value))?;
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
    pub fn get_RequiresSave(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_RequiresSave", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_PluginStorage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage+DataType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_PluginStorage_DataType {
    BOOL = 0i32,
    FLOAT = 3i32,
    INT = 1i32,
    LONG = 2i32,
    STRING = 4i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage+DataType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_PluginStorage_DataType
    => "HoudiniEngineUnity"."HEU_PluginStorage/DataType"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage+StoreData")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_PluginStorage_StoreData {
    __cordl_parent: crate::System::Object,
    pub _type: crate::HoudiniEngineUnity::HEU_PluginStorage_DataType,
    pub _valueStr: *mut crate::System::String,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage+StoreData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_PluginStorage_StoreData
    => "HoudiniEngineUnity"."HEU_PluginStorage/StoreData"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage+StoreData")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_PluginStorage_StoreData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage+StoreData")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_PluginStorage_StoreData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage+StoreData")]
impl crate::HoudiniEngineUnity::HEU_PluginStorage_StoreData {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage+StoreData")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_PluginStorage_StoreData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage+StoreDataArray_1")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_PluginStorage_StoreDataArray_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Object,
    pub _array: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage+StoreDataArray_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_PluginStorage_StoreDataArray_1 < T > =>
    "HoudiniEngineUnity"."HEU_PluginStorage/StoreDataArray`1" < T >
);
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage+StoreDataArray_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::HoudiniEngineUnity::HEU_PluginStorage_StoreDataArray_1<T> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage+StoreDataArray_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_PluginStorage_StoreDataArray_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage+StoreDataArray_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::HoudiniEngineUnity::HEU_PluginStorage_StoreDataArray_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginStorage+StoreDataArray_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_PluginStorage_StoreDataArray_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
