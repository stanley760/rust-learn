// Model registry for managing multiple BERT models

use crate::core::BertModel;
use crate::utils::AppError;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Model registry for managing multiple loaded models
///
/// The registry allows:
/// - Registering multiple models with unique IDs
/// - Activating a specific model as the active one
/// - Querying the currently active model
/// - Listing all registered models
pub struct ModelRegistry {
    active_model: Option<Arc<RwLock<BertModel>>>,
    models: HashMap<String, Arc<RwLock<BertModel>>>,
}

impl ModelRegistry {
    /// Create a new empty ModelRegistry
    pub fn new() -> Self {
        tracing::info!("Creating new ModelRegistry");
        Self {
            active_model: None,
            models: HashMap::new(),
        }
    }

    /// Register a model with a unique ID
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the model
    /// * `model` - The BERT model to register
    pub fn register(&mut self, id: String, model: Arc<RwLock<BertModel>>) {
        tracing::info!("Registering model with ID: {}", id);
        self.models.insert(id.clone(), Arc::clone(&model));

        // If this is the first model, make it active
        if self.active_model.is_none() {
            self.active_model = Some(model);
            tracing::info!("Model '{}' is now the active model", id);
        }
    }

    /// Activate a previously registered model
    ///
    /// # Arguments
    /// * `id` - The ID of the model to activate
    ///
    /// # Returns
    /// * `Result<()>` - Success or error if model not found
    pub fn activate(&mut self, id: &str) -> Result<(), AppError> {
        tracing::info!("Activating model with ID: {}", id);

        let model = self.models.get(id).ok_or_else(|| {
            AppError::ModelError(format!("Model not found: {}", id))
        })?;

        self.active_model = Some(Arc::clone(model));
        tracing::info!("Model '{}' is now the active model", id);

        Ok(())
    }

    /// Get the currently active model
    ///
    /// # Returns
    /// * `Option<Arc<RwLock<BertModel>>>` - The active model, or None if no model is active
    pub fn get_active(&self) -> Option<Arc<RwLock<BertModel>>> {
        self.active_model.as_ref().map(Arc::clone)
    }

    /// Get a specific model by ID
    ///
    /// # Arguments
    /// * `id` - The ID of the model to get
    ///
    /// # Returns
    /// * `Result<Arc<RwLock<BertModel>>>` - The model, or error if not found
    pub fn get(&self, id: &str) -> Result<Arc<RwLock<BertModel>>, AppError> {
        self.models.get(id)
            .cloned()
            .ok_or_else(|| AppError::ModelError(format!("Model not found: {}", id)))
    }

    /// List all registered model IDs
    ///
    /// # Returns
    /// * `Vec<String>` - List of registered model IDs
    pub fn list(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.models.keys().cloned().collect();
        ids.sort(); // Sort for consistent ordering
        ids
    }

    /// Check if a model is currently active
    ///
    /// # Returns
    /// * `bool` - True if a model is active, false otherwise
    pub fn has_active(&self) -> bool {
        self.active_model.is_some()
    }

    /// Get the ID of the currently active model
    ///
    /// # Returns
    /// * `Option<String>` - The ID of the active model, or None if no model is active
    pub fn active_id(&self) -> Option<String> {
        // Find the ID that matches the active model
        for (id, model) in &self.models {
            if let Some(active) = &self.active_model {
                // Compare Arc addresses
                if Arc::ptr_eq(model, active) {
                    return Some(id.clone());
                }
            }
        }
        None
    }

    /// Unregister a model
    ///
    /// # Arguments
    /// * `id` - The ID of the model to unregister
    ///
    /// # Returns
    /// * `Result<()>` - Success or error if model not found
    ///
    /// # Note
    /// If the unregistered model is currently active, the active model will be cleared
    pub fn unregister(&mut self, id: &str) -> Result<(), AppError> {
        tracing::info!("Unregistering model with ID: {}", id);

        let removed = self.models.remove(id).ok_or_else(|| {
            AppError::ModelError(format!("Model not found: {}", id))
        })?;

        // If the removed model was active, clear the active model
        if let Some(active) = &self.active_model {
            if Arc::ptr_eq(&removed, active) {
                self.active_model = None;
                tracing::info!("Active model was unregistered");
            }
        }

        Ok(())
    }

    /// Unregister all models
    pub fn unregister_all(&mut self) {
        tracing::info!("Unregistering all models");
        self.models.clear();
        self.active_model = None;
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::{Device, DType};
    use candle_nn::VarBuilder;
    use candle_transformers::models::bert::Config as BertConfig;

    fn create_test_model() -> BertModel {
        let config = BertConfig {
            vocab_size: 1000,
            hidden_size: 128,
            num_hidden_layers: 2,
            num_attention_heads: 2,
            intermediate_size: 512,
            hidden_act: candle_transformers::models::bert::HiddenAct::Gelu,
            hidden_dropout_prob: 0.1,
            max_position_embeddings: 128,
            type_vocab_size: 2,
            initializer_range: 0.02,
            layer_norm_eps: 1e-12,
            pad_token_id: 0,
            position_embedding_type: candle_transformers::models::bert::PositionEmbeddingType::Absolute,
            use_cache: false,
            classifier_dropout: None,
            model_type: None,
        };
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);
        BertModel::load(vb, &config, device).unwrap()
    }

    #[test]
    fn test_registry_creation() {
        let registry = ModelRegistry::new();
        assert!(!registry.has_active());
        assert!(registry.list().is_empty());
    }

    #[test]
    fn test_register_first_model_becomes_active() {
        let mut registry = ModelRegistry::new();
        let model = Arc::new(RwLock::new(create_test_model()));

        registry.register("model1".to_string(), model);

        assert!(registry.has_active());
        assert_eq!(registry.active_id(), Some("model1".to_string()));
    }

    #[test]
    fn test_register_multiple_models() {
        let mut registry = ModelRegistry::new();
        let model1 = Arc::new(RwLock::new(create_test_model()));
        let model2 = Arc::new(RwLock::new(create_test_model()));

        registry.register("model1".to_string(), model1);
        registry.register("model2".to_string(), model2);

        let ids = registry.list();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&"model1".to_string()));
        assert!(ids.contains(&"model2".to_string()));
    }

    #[test]
    fn test_activate_model() {
        let mut registry = ModelRegistry::new();
        let model1 = Arc::new(RwLock::new(create_test_model()));
        let model2 = Arc::new(RwLock::new(create_test_model()));

        registry.register("model1".to_string(), model1);
        registry.register("model2".to_string(), model2);

        // First model should be active
        assert_eq!(registry.active_id(), Some("model1".to_string()));

        // Activate second model
        registry.activate("model2").unwrap();
        assert_eq!(registry.active_id(), Some("model2".to_string()));
    }

    #[test]
    fn test_activate_nonexistent_model() {
        let mut registry = ModelRegistry::new();
        let result = registry.activate("nonexistent");

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ModelError(_)));
    }

    #[test]
    fn test_get_model() {
        let mut registry = ModelRegistry::new();
        let model1 = Arc::new(RwLock::new(create_test_model()));

        registry.register("model1".to_string(), Arc::clone(&model1));

        let retrieved = registry.get("model1").unwrap();
        assert!(Arc::ptr_eq(&model1, &retrieved));
    }

    #[test]
    fn test_get_nonexistent_model() {
        let registry = ModelRegistry::new();
        let result = registry.get("nonexistent");

        // Check that an error is returned without requiring Debug trait
        assert!(result.is_err());
        if let Err(AppError::ModelError(msg)) = result {
            assert!(msg.contains("not found"));
        } else {
            panic!("Expected ModelError with 'not found' message");
        }
    }

    #[test]
    fn test_unregister_active_model() {
        let mut registry = ModelRegistry::new();
        let model1 = Arc::new(RwLock::new(create_test_model()));
        let model2 = Arc::new(RwLock::new(create_test_model()));

        registry.register("model1".to_string(), model1);
        registry.register("model2".to_string(), model2);

        assert_eq!(registry.active_id(), Some("model1".to_string()));

        // Unregister active model
        registry.unregister("model1").unwrap();

        // Should have no active model now
        assert!(!registry.has_active());
        assert_eq!(registry.active_id(), None);
    }

    #[test]
    fn test_unregister_inactive_model() {
        let mut registry = ModelRegistry::new();
        let model1 = Arc::new(RwLock::new(create_test_model()));
        let model2 = Arc::new(RwLock::new(create_test_model()));

        registry.register("model1".to_string(), model1);
        registry.register("model2".to_string(), model2);

        assert_eq!(registry.active_id(), Some("model1".to_string()));

        // Unregister inactive model
        registry.unregister("model2").unwrap();

        // Active model should still be model1
        assert_eq!(registry.active_id(), Some("model1".to_string()));
    }

    #[test]
    fn test_unregister_all() {
        let mut registry = ModelRegistry::new();
        let model1 = Arc::new(RwLock::new(create_test_model()));
        let model2 = Arc::new(RwLock::new(create_test_model()));

        registry.register("model1".to_string(), model1);
        registry.register("model2".to_string(), model2);

        registry.unregister_all();

        assert!(!registry.has_active());
        assert!(registry.list().is_empty());
    }

    #[test]
    fn test_default() {
        let registry = ModelRegistry::default();
        assert!(!registry.has_active());
        assert!(registry.list().is_empty());
    }
}
