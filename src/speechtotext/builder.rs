use std::sync::Arc;

use crate::errors::Result;
use crate::{
    api::grpc::google::cloud::speechtotext::v1::StreamingRecognitionConfig, common::get_token,
};

use super::recognizer::Recognizer;

/// Google Speech API recognizer builder
#[derive(Clone, Debug)]
pub struct RecognizerBuilder {
    token: Arc<String>,
}

impl RecognizerBuilder {
    pub fn new(
        // Google Cloud Platform JSON credentials for project with Speech APIs enabled
        google_credentials: impl AsRef<str>,
    ) -> Result<Self> {
        Ok(Self {
            token: get_token(google_credentials)?,
        })
    }

    /// Creates new speech recognizer from provided
    /// Google credentials and google speech configuration.
    /// This kind of recognizer can be used for streaming recognition.
    pub async fn create_streaming_recognizer(
        &self,
        //  Streaming recognition configuration
        config: StreamingRecognitionConfig,
        // Capacity of audio sink (tokio channel used by caller to send audio data).
        // If not provided defaults to 1000.
        buffer_size: Option<usize>,
    ) -> Result<Recognizer> {
        Recognizer::create_streaming_recognizer_with_token(self.token.clone(), config, buffer_size)
            .await
    }

    /// Creates new speech recognizer from provided
    /// Google credentials. This kind of recognizer can be used
    /// for long running recognition.
    pub async fn create_asynchronous_recognizer(&self) -> Result<Recognizer> {
        Recognizer::create_synchronous_recognizer_with_token(self.token.clone()).await
    }

    /// Creates new speech recognizer from provided
    /// Google credentials. This kind of recognizer can be used
    /// for synchronous recognition.
    pub async fn create_synchronous_recognizer(&self) -> Result<Recognizer> {
        Recognizer::create_synchronous_recognizer_with_token(self.token.clone()).await
    }
}
