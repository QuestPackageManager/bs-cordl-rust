#[cfg(feature = "OVRMesh")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRMesh {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _dataProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRMesh_IOVRMeshDataProvider,
    >,
    pub _meshType: crate::GlobalNamespace::OVRMesh_MeshType,
    pub _mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    pub _IsInitialized_k__BackingField: bool,
}
#[cfg(feature = "OVRMesh")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRMesh => ""."OVRMesh"
);
#[cfg(feature = "OVRMesh")]
impl std::ops::Deref for crate::GlobalNamespace::OVRMesh {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMesh")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRMesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMesh")]
impl crate::GlobalNamespace::OVRMesh {
    #[cfg(feature = "OVRMesh+IOVRMeshDataProvider")]
    type IOVRMeshDataProvider = crate::GlobalNamespace::OVRMesh_IOVRMeshDataProvider;
    #[cfg(feature = "OVRMesh+MeshType")]
    pub type MeshType = crate::GlobalNamespace::OVRMesh_MeshType;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
        meshType: crate::GlobalNamespace::OVRMesh_MeshType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (meshType))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetMeshType(
        &mut self,
        _cordl_type: crate::GlobalNamespace::OVRMesh_MeshType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMeshType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldInitialize(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldInitialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TransformOvrpMesh(
        &mut self,
        ovrpMesh: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRPlugin_Mesh>,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransformOvrpMesh", (ovrpMesh, mesh))?;
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
    pub fn get_IsInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Mesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = __cordl_object
            .invoke("get_Mesh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsInitialized(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsInitialized", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRMesh")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRMesh {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRMesh+IOVRMeshDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRMesh_IOVRMeshDataProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRMesh+IOVRMeshDataProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRMesh_IOVRMeshDataProvider =>
    ""."OVRMesh/IOVRMeshDataProvider"
);
#[cfg(feature = "OVRMesh+IOVRMeshDataProvider")]
impl std::ops::Deref for crate::GlobalNamespace::OVRMesh_IOVRMeshDataProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMesh+IOVRMeshDataProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRMesh_IOVRMeshDataProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMesh+IOVRMeshDataProvider")]
impl crate::GlobalNamespace::OVRMesh_IOVRMeshDataProvider {
    pub fn GetMeshType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRMesh_MeshType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRMesh_MeshType = __cordl_object
            .invoke("GetMeshType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "OVRMesh+IOVRMeshDataProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRMesh_IOVRMeshDataProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRMesh+MeshType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRMesh_MeshType {
    HandLeft = 0i32,
    HandRight = 1i32,
    None = -1i32,
}
#[cfg(feature = "OVRMesh+MeshType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRMesh_MeshType => ""
    ."OVRMesh/MeshType"
);
