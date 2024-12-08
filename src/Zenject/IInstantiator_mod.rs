#[cfg(feature = "Zenject+IInstantiator")]
#[repr(C)]
#[derive(Debug)]
pub struct IInstantiator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+IInstantiator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::IInstantiator => "Zenject"
    ."IInstantiator"
);
#[cfg(feature = "Zenject+IInstantiator")]
impl std::ops::Deref for crate::Zenject::IInstantiator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IInstantiator")]
impl std::ops::DerefMut for crate::Zenject::IInstantiator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IInstantiator")]
impl crate::Zenject::IInstantiator {
    pub fn InstantiateComponentOnNewGameObject_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiateComponentOnNewGameObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateComponentOnNewGameObject_String1<T>(
        &mut self,
        gameObjectName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiateComponentOnNewGameObject", (gameObjectName))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateComponentOnNewGameObject_IEnumerable_1_2<T>(
        &mut self,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiateComponentOnNewGameObject", (extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateComponentOnNewGameObject_String_IEnumerable_1_3<T>(
        &mut self,
        gameObjectName: *mut crate::System::String,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiateComponentOnNewGameObject", (gameObjectName, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResource_String0(
        &mut self,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("InstantiatePrefabResource", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResource_Transform1(
        &mut self,
        resourcePath: *mut crate::System::String,
        parentTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("InstantiatePrefabResource", (resourcePath, parentTransform))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResource_Vector3_Quaternion_Transform2(
        &mut self,
        resourcePath: *mut crate::System::String,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke(
                "InstantiatePrefabResource",
                (resourcePath, position, rotation, parentTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateEmptyGameObject(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("CreateEmptyGameObject", (name))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateComponent_GameObject0<TContract>(
        &mut self,
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<TContract>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TContract = __cordl_object
            .invoke("InstantiateComponent", (gameObject))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateComponent_GameObject_IEnumerable_1_1<TContract>(
        &mut self,
        gameObject: *mut crate::UnityEngine::GameObject,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<TContract>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TContract = __cordl_object
            .invoke("InstantiateComponent", (gameObject, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateComponent_Type_GameObject2(
        &mut self,
        componentType: *mut crate::System::Type,
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("InstantiateComponent", (componentType, gameObject))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateComponent_Type_GameObject_IEnumerable_1_3(
        &mut self,
        componentType: *mut crate::System::Type,
        gameObject: *mut crate::UnityEngine::GameObject,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("InstantiateComponent", (componentType, gameObject, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateScriptableObjectResource_String0<T>(
        &mut self,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiateScriptableObjectResource", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateScriptableObjectResource_String_IEnumerable_1_1<T>(
        &mut self,
        resourcePath: *mut crate::System::String,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiateScriptableObjectResource", (resourcePath, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateScriptableObjectResource_Type_String2(
        &mut self,
        scriptableObjectType: *mut crate::System::Type,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InstantiateScriptableObjectResource",
                (scriptableObjectType, resourcePath),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateScriptableObjectResource_Type_String_IEnumerable_1_3(
        &mut self,
        scriptableObjectType: *mut crate::System::Type,
        resourcePath: *mut crate::System::String,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InstantiateScriptableObjectResource",
                (scriptableObjectType, resourcePath, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefab_Object0(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("InstantiatePrefab", (prefab))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefab_Transform1(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        parentTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("InstantiatePrefab", (prefab, parentTransform))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefab_Vector3_Quaternion_Transform2(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("InstantiatePrefab", (prefab, position, rotation, parentTransform))?;
        Ok(__cordl_ret)
    }
    pub fn Instantiate_0<T>(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("Instantiate", ())?;
        Ok(__cordl_ret)
    }
    pub fn Instantiate_IEnumerable_1_1<T>(
        &mut self,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("Instantiate", (extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn Instantiate_Type2(
        &mut self,
        concreteType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Instantiate", (concreteType))?;
        Ok(__cordl_ret)
    }
    pub fn Instantiate_Type_IEnumerable_1_3(
        &mut self,
        concreteType: *mut crate::System::Type,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Instantiate", (concreteType, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponent_Object0<T>(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiatePrefabForComponent", (prefab))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponent_Object_IEnumerable_1_1<T>(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiatePrefabForComponent", (prefab, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponent_Object_Transform2<T>(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        parentTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiatePrefabForComponent", (prefab, parentTransform))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponent_Object_Transform_IEnumerable_1_3<T>(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        parentTransform: *mut crate::UnityEngine::Transform,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke(
                "InstantiatePrefabForComponent",
                (prefab, parentTransform, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponent_Object_Vector3_Quaternion_Transform4<T>(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke(
                "InstantiatePrefabForComponent",
                (prefab, position, rotation, parentTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponent_Object_Vector3_Quaternion_Transform_IEnumerable_1_5<
        T,
    >(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: *mut crate::UnityEngine::Transform,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke(
                "InstantiatePrefabForComponent",
                (prefab, position, rotation, parentTransform, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabForComponent_Type_Object_Transform_IEnumerable_1_6(
        &mut self,
        concreteType: *mut crate::System::Type,
        prefab: *mut crate::UnityEngine::Object,
        parentTransform: *mut crate::UnityEngine::Transform,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InstantiatePrefabForComponent",
                (concreteType, prefab, parentTransform, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResourceForComponent_String0<T>(
        &mut self,
        resourcePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiatePrefabResourceForComponent", (resourcePath))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResourceForComponent_String_IEnumerable_1_1<T>(
        &mut self,
        resourcePath: *mut crate::System::String,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InstantiatePrefabResourceForComponent", (resourcePath, extraArgs))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResourceForComponent_String_Transform2<T>(
        &mut self,
        resourcePath: *mut crate::System::String,
        parentTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke(
                "InstantiatePrefabResourceForComponent",
                (resourcePath, parentTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResourceForComponent_String_Transform_IEnumerable_1_3<T>(
        &mut self,
        resourcePath: *mut crate::System::String,
        parentTransform: *mut crate::UnityEngine::Transform,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke(
                "InstantiatePrefabResourceForComponent",
                (resourcePath, parentTransform, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResourceForComponent_String_Vector3_Quaternion_Transform4<T>(
        &mut self,
        resourcePath: *mut crate::System::String,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke(
                "InstantiatePrefabResourceForComponent",
                (resourcePath, position, rotation, parentTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResourceForComponent_String_Vector3_Quaternion_Transform_IEnumerable_1_5<
        T,
    >(
        &mut self,
        resourcePath: *mut crate::System::String,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: *mut crate::UnityEngine::Transform,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke(
                "InstantiatePrefabResourceForComponent",
                (resourcePath, position, rotation, parentTransform, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiatePrefabResourceForComponent_Type_String_Transform_IEnumerable_1_6(
        &mut self,
        concreteType: *mut crate::System::Type,
        resourcePath: *mut crate::System::String,
        parentTransform: *mut crate::UnityEngine::Transform,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InstantiatePrefabResourceForComponent",
                (concreteType, resourcePath, parentTransform, extraArgs),
            )?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Zenject+IInstantiator")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::IInstantiator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
