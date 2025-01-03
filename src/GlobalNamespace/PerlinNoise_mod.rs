#[cfg(feature = "PerlinNoise")]
#[repr(C)]
#[derive(Debug)]
pub struct PerlinNoise {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "PerlinNoise")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PerlinNoise => ""."PerlinNoise"
);
#[cfg(feature = "PerlinNoise")]
impl std::ops::Deref for crate::GlobalNamespace::PerlinNoise {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PerlinNoise")]
impl std::ops::DerefMut for crate::GlobalNamespace::PerlinNoise {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PerlinNoise")]
impl crate::GlobalNamespace::PerlinNoise {
    pub fn Fade(t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Fade", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn Grad3D(
        hash: i32,
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Grad3D", (hash, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn Inc(num: i32, repeat: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Inc", (num, repeat))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lerp(a: f32, b: f32, x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lerp", (a, b, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OctavePerlin3D(
        x: f32,
        y: f32,
        z: f32,
        octaves: i32,
        persistence: f32,
        repeat: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OctavePerlin3D", (x, y, z, octaves, persistence, repeat))?;
        Ok(__cordl_ret.into())
    }
    pub fn Perlin3D(
        x: f32,
        y: f32,
        z: f32,
        repeat: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Perlin3D", (x, y, z, repeat))?;
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
#[cfg(feature = "PerlinNoise")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PerlinNoise {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
