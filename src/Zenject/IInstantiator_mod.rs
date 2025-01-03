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
    pub fn CreateEmptyGameObject(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("CreateEmptyGameObject", (name))?;
        Ok(__cordl_ret.into())
    }
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateComponentOnNewGameObject_IEnumerable_1_2<T>(
        &mut self,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateComponentOnNewGameObject_Il2CppString1<T>(
        &mut self,
        gameObjectName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateComponentOnNewGameObject_Il2CppString_IEnumerable_1_3<T>(
        &mut self,
        gameObjectName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateComponent_GameObject0<TContract>(
        &mut self,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateComponent_GameObject_IEnumerable_1_1<TContract>(
        &mut self,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateComponent_Type_GameObject2(
        &mut self,
        componentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component> = __cordl_object
            .invoke("InstantiateComponent", (componentType, gameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateComponent_Type_GameObject_IEnumerable_1_3(
        &mut self,
        componentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component> = __cordl_object
            .invoke("InstantiateComponent", (componentType, gameObject, extraArgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefabForComponent_Object0<T>(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefabForComponent_Object_IEnumerable_1_1<T>(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefabForComponent_Object_Transform2<T>(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefabForComponent_Object_Transform_IEnumerable_1_3<T>(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefabForComponent_Object_Vector3_Quaternion_Transform4<T>(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefabForComponent_Object_Vector3_Quaternion_Transform_IEnumerable_1_5<
        T,
    >(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefabForComponent_Type_Object_Transform_IEnumerable_1_6(
        &mut self,
        concreteType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke(
                "InstantiatePrefabForComponent",
                (concreteType, prefab, parentTransform, extraArgs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefabResourceForComponent_Il2CppString0<T>(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefabResourceForComponent_Il2CppString_IEnumerable_1_1<T>(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefabResourceForComponent_Il2CppString_Transform2<T>(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefabResourceForComponent_Il2CppString_Transform_IEnumerable_1_3<
        T,
    >(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefabResourceForComponent_Il2CppString_Vector3_Quaternion_Transform4<
        T,
    >(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefabResourceForComponent_Il2CppString_Vector3_Quaternion_Transform_IEnumerable_1_5<
        T,
    >(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefabResourceForComponent_Type_Il2CppString_Transform_IEnumerable_1_6(
        &mut self,
        concreteType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke(
                "InstantiatePrefabResourceForComponent",
                (concreteType, resourcePath, parentTransform, extraArgs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefabResource_Il2CppString0(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("InstantiatePrefabResource", (resourcePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefabResource_Transform1(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("InstantiatePrefabResource", (resourcePath, parentTransform))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefabResource_Vector3_Quaternion_Transform2(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke(
                "InstantiatePrefabResource",
                (resourcePath, position, rotation, parentTransform),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefab_Object0(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("InstantiatePrefab", (prefab))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefab_Transform1(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("InstantiatePrefab", (prefab, parentTransform))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiatePrefab_Vector3_Quaternion_Transform2(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("InstantiatePrefab", (prefab, position, rotation, parentTransform))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateScriptableObjectResource_Il2CppString0<T>(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateScriptableObjectResource_Il2CppString_IEnumerable_1_1<T>(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateScriptableObjectResource_Type_Il2CppString2(
        &mut self,
        scriptableObjectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke(
                "InstantiateScriptableObjectResource",
                (scriptableObjectType, resourcePath),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateScriptableObjectResource_Type_Il2CppString_IEnumerable_1_3(
        &mut self,
        scriptableObjectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke(
                "InstantiateScriptableObjectResource",
                (scriptableObjectType, resourcePath, extraArgs),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_IEnumerable_1_1<T>(
        &mut self,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_Type2(
        &mut self,
        concreteType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Instantiate", (concreteType))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_Type_IEnumerable_1_3(
        &mut self,
        concreteType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Instantiate", (concreteType, extraArgs))?;
        Ok(__cordl_ret.into())
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
