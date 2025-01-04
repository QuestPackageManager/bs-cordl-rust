#[cfg(feature = "UnityEngine+Rendering+RenderingThreadingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderingThreadingMode {
    #[default]
    Direct = 0i32,
    LegacyJobified = 3i32,
    MultiThreaded = 2i32,
    NativeGraphicsJobs = 4i32,
    NativeGraphicsJobsWithoutRenderThread = 5i32,
    SingleThreaded = 1i32,
}
#[cfg(feature = "UnityEngine+Rendering+RenderingThreadingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::RenderingThreadingMode
    => "UnityEngine.Rendering"."RenderingThreadingMode"
);
