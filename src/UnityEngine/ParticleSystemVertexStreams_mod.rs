#[cfg(feature = "UnityEngine+ParticleSystemVertexStreams")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleSystemVertexStreams {
    All = 2147483647i32,
    CenterAndVertexID = 64i32,
    Color = 8i32,
    Custom1 = 2048i32,
    Custom2 = 4096i32,
    Lifetime = 1024i32,
    None = 0i32,
    Normal = 2i32,
    Position = 1i32,
    Random = 8192i32,
    Rotation = 256i32,
    Size = 128i32,
    Tangent = 4i32,
    UV = 16i32,
    UV2BlendAndFrame = 32i32,
    Velocity = 512i32,
}
#[cfg(feature = "UnityEngine+ParticleSystemVertexStreams")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystemVertexStreams =>
    "UnityEngine"."ParticleSystemVertexStreams"
);
