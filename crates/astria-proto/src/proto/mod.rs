#[path = "tonic"]
/// Files generated using [`prost`] and [`tonic`] via [`buf`] and its
/// [`neoeinstein-prost`] and [`neoeinstein-tonic`] plugins.
///
/// [`prost`]:
/// [`tonic`]:
/// [`buf`]: https://buf.build
/// [`neoeinstein-prost`]: https://buf.build/community/neoeinstein-prost
/// [`neoeinstein-tonic`]: https://buf.build/community/neoeinstein-tonic
pub mod tonic {
    #[path = ""]
    pub mod execution {
        #[path = "astria.execution.v1.rs"]
        pub mod v1;
    }

    #[path = ""]
    pub mod primitive {
        #[path = "astria.primitive.v1.rs"]
        pub mod v1;
    }

    #[path = ""]
    pub mod sequencer {
        #[path = "astria.sequencer.v1.rs"]
        pub mod v1;
    }
}