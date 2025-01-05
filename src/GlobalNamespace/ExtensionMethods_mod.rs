#[cfg(feature = "ExtensionMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct ExtensionMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ExtensionMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ExtensionMethods => ""
    ."ExtensionMethods"
);
#[cfg(feature = "ExtensionMethods")]
impl std::ops::Deref for crate::GlobalNamespace::ExtensionMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ExtensionMethods")]
impl std::ops::DerefMut for crate::GlobalNamespace::ExtensionMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ExtensionMethods")]
impl crate::GlobalNamespace::ExtensionMethods {
    pub fn ContainsLayer(
        layerMask: crate::UnityEngine::LayerMask,
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsLayer", (layerMask, layer))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTexture2D(
        renderTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        textureFormat: crate::UnityEngine::TextureFormat,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateTexture2D", (renderTexture, textureFormat))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPath(
        current: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetPath", (current))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRange<T>(
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<T>,
        >,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRange", (list, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDescendantOf(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDescendantOf", (transform, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn LastUnsafe<T>(
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LastUnsafe", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reflect(
        source: crate::UnityEngine::Quaternion,
        normal: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Reflect", (source, normal))?;
        Ok(__cordl_ret.into())
    }
    pub fn Rotate(
        vector: crate::UnityEngine::Vector2,
        rads: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Rotate", (vector, rads))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPositionAndRotation(
        tr: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        pos: crate::UnityEngine::Vector3,
        rot: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetLocalPositionAndRotation", (tr, pos, rot))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSeed(
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
        seed: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSeed", (particleSystem, seed))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartUniqueCoroutine_Func_1_0(
        m: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
        func: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StartUniqueCoroutine", (m, func))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartUniqueCoroutine_Func_2_T1<T>(
        m: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
        func: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                T,
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
            >,
        >,
        value: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StartUniqueCoroutine", (m, func, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn StopUniqueCoroutine_Func_1_0(
        m: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
        func: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StopUniqueCoroutine", (m, func))?;
        Ok(__cordl_ret.into())
    }
    pub fn StopUniqueCoroutine_Func_2_1<T>(
        m: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
        func: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                T,
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StopUniqueCoroutine", (m, func))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ExtensionMethods")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ExtensionMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
