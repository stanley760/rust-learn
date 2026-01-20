// Fine-tuning task management module
// Requirements: 6.3, 6.4

use crate::api::models::FinetuneStatus;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Information about a fine-tuning job
#[derive(Debug, Clone)]
pub struct FinetuneJob {
    pub job_id: String,
    pub status: FinetuneStatus,
    pub progress: f32,
    pub current_epoch: usize,
    pub total_epochs: usize,
    pub current_loss: Option<f32>,
    pub error_message: Option<String>,
}

impl FinetuneJob {
    /// Create a new pending fine-tuning job
    pub fn new(job_id: String, total_epochs: usize) -> Self {
        Self {
            job_id,
            status: FinetuneStatus::Pending,
            progress: 0.0,
            current_epoch: 0,
            total_epochs,
            current_loss: None,
            error_message: None,
        }
    }

    /// Update job status to running
    pub fn set_running(&mut self) {
        self.status = FinetuneStatus::Running;
    }

    /// Update job progress
    pub fn update_progress(&mut self, current_epoch: usize, loss: Option<f32>) {
        self.current_epoch = current_epoch;
        self.current_loss = loss;
        self.progress = if self.total_epochs > 0 {
            (current_epoch as f32 / self.total_epochs as f32) * 100.0
        } else {
            0.0
        };
    }

    /// Mark job as completed
    pub fn set_completed(&mut self) {
        self.status = FinetuneStatus::Completed;
        self.progress = 100.0;
    }

    /// Mark job as failed with error message
    pub fn set_failed(&mut self, error_message: String) {
        self.status = FinetuneStatus::Failed;
        self.error_message = Some(error_message);
    }
}

/// Manager for fine-tuning jobs
/// 
/// Uses Arc<Mutex<HashMap>> to store job states in a thread-safe manner
/// Requirements: 6.3, 6.4
#[derive(Clone)]
pub struct FinetuneJobManager {
    jobs: Arc<Mutex<HashMap<String, FinetuneJob>>>,
}

impl FinetuneJobManager {
    /// Create a new FinetuneJobManager
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Generate a new unique job ID using UUID
    /// 
    /// Requirements: 6.3
    pub fn generate_job_id() -> String {
        Uuid::new_v4().to_string()
    }

    /// Create a new fine-tuning job
    /// 
    /// # Arguments
    /// * `total_epochs` - Total number of training epochs
    /// 
    /// # Returns
    /// * `String` - The generated job ID
    /// 
    /// Requirements: 6.3
    pub fn create_job(&self, total_epochs: usize) -> String {
        let job_id = Self::generate_job_id();
        let job = FinetuneJob::new(job_id.clone(), total_epochs);

        let mut jobs = self.jobs.lock().unwrap();
        jobs.insert(job_id.clone(), job);

        tracing::info!("Created fine-tuning job: {}", job_id);

        job_id
    }

    /// Get the status of a fine-tuning job
    /// 
    /// # Arguments
    /// * `job_id` - The job ID to query
    /// 
    /// # Returns
    /// * `Option<FinetuneJob>` - The job information if it exists
    /// 
    /// Requirements: 6.4
    pub fn get_job(&self, job_id: &str) -> Option<FinetuneJob> {
        let jobs = self.jobs.lock().unwrap();
        jobs.get(job_id).cloned()
    }

    /// Update a job's status to running
    /// 
    /// # Arguments
    /// * `job_id` - The job ID to update
    pub fn set_job_running(&self, job_id: &str) {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(job) = jobs.get_mut(job_id) {
            job.set_running();
            tracing::info!("Job {} status updated to running", job_id);
        }
    }

    /// Update a job's progress
    /// 
    /// # Arguments
    /// * `job_id` - The job ID to update
    /// * `current_epoch` - Current epoch number
    /// * `loss` - Current loss value (optional)
    pub fn update_job_progress(&self, job_id: &str, current_epoch: usize, loss: Option<f32>) {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(job) = jobs.get_mut(job_id) {
            job.update_progress(current_epoch, loss);
            tracing::debug!(
                "Job {} progress updated: epoch {}/{}, loss: {:?}",
                job_id,
                current_epoch,
                job.total_epochs,
                loss
            );
        }
    }

    /// Mark a job as completed
    /// 
    /// # Arguments
    /// * `job_id` - The job ID to update
    pub fn set_job_completed(&self, job_id: &str) {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(job) = jobs.get_mut(job_id) {
            job.set_completed();
            tracing::info!("Job {} completed successfully", job_id);
        }
    }

    /// Mark a job as failed
    /// 
    /// # Arguments
    /// * `job_id` - The job ID to update
    /// * `error_message` - Error message describing the failure
    pub fn set_job_failed(&self, job_id: &str, error_message: String) {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(job) = jobs.get_mut(job_id) {
            job.set_failed(error_message.clone());
            tracing::error!("Job {} failed: {}", job_id, error_message);
        }
    }

    /// Remove a job from the manager
    /// 
    /// # Arguments
    /// * `job_id` - The job ID to remove
    pub fn remove_job(&self, job_id: &str) {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.remove(job_id);
        tracing::info!("Job {} removed from manager", job_id);
    }

    /// Get all jobs
    /// 
    /// # Returns
    /// * `Vec<FinetuneJob>` - List of all jobs
    pub fn list_jobs(&self) -> Vec<FinetuneJob> {
        let jobs = self.jobs.lock().unwrap();
        jobs.values().cloned().collect()
    }
}

impl Default for FinetuneJobManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_job_id_is_unique() {
        let id1 = FinetuneJobManager::generate_job_id();
        let id2 = FinetuneJobManager::generate_job_id();

        assert_ne!(id1, id2);
        assert!(!id1.is_empty());
        assert!(!id2.is_empty());
    }

    #[test]
    fn test_create_job() {
        let manager = FinetuneJobManager::new();
        let job_id = manager.create_job(5);

        assert!(!job_id.is_empty());

        let job = manager.get_job(&job_id);
        assert!(job.is_some());

        let job = job.unwrap();
        assert_eq!(job.job_id, job_id);
        assert_eq!(job.status, FinetuneStatus::Pending);
        assert_eq!(job.total_epochs, 5);
        assert_eq!(job.current_epoch, 0);
        assert_eq!(job.progress, 0.0);
    }

    #[test]
    fn test_get_nonexistent_job() {
        let manager = FinetuneJobManager::new();
        let job = manager.get_job("nonexistent-id");

        assert!(job.is_none());
    }

    #[test]
    fn test_set_job_running() {
        let manager = FinetuneJobManager::new();
        let job_id = manager.create_job(3);

        manager.set_job_running(&job_id);

        let job = manager.get_job(&job_id).unwrap();
        assert_eq!(job.status, FinetuneStatus::Running);
    }

    #[test]
    fn test_update_job_progress() {
        let manager = FinetuneJobManager::new();
        let job_id = manager.create_job(10);

        manager.update_job_progress(&job_id, 5, Some(0.25));

        let job = manager.get_job(&job_id).unwrap();
        assert_eq!(job.current_epoch, 5);
        assert_eq!(job.current_loss, Some(0.25));
        assert_eq!(job.progress, 50.0);
    }

    #[test]
    fn test_set_job_completed() {
        let manager = FinetuneJobManager::new();
        let job_id = manager.create_job(3);

        manager.set_job_completed(&job_id);

        let job = manager.get_job(&job_id).unwrap();
        assert_eq!(job.status, FinetuneStatus::Completed);
        assert_eq!(job.progress, 100.0);
    }

    #[test]
    fn test_set_job_failed() {
        let manager = FinetuneJobManager::new();
        let job_id = manager.create_job(3);

        let error_msg = "Training failed due to invalid data".to_string();
        manager.set_job_failed(&job_id, error_msg.clone());

        let job = manager.get_job(&job_id).unwrap();
        assert_eq!(job.status, FinetuneStatus::Failed);
        assert_eq!(job.error_message, Some(error_msg));
    }

    #[test]
    fn test_remove_job() {
        let manager = FinetuneJobManager::new();
        let job_id = manager.create_job(3);

        assert!(manager.get_job(&job_id).is_some());

        manager.remove_job(&job_id);

        assert!(manager.get_job(&job_id).is_none());
    }

    #[test]
    fn test_list_jobs() {
        let manager = FinetuneJobManager::new();

        let job_id1 = manager.create_job(3);
        let job_id2 = manager.create_job(5);

        let jobs = manager.list_jobs();
        assert_eq!(jobs.len(), 2);

        let job_ids: Vec<String> = jobs.iter().map(|j| j.job_id.clone()).collect();
        assert!(job_ids.contains(&job_id1));
        assert!(job_ids.contains(&job_id2));
    }

    #[test]
    fn test_finetune_job_new() {
        let job = FinetuneJob::new("test-id".to_string(), 5);

        assert_eq!(job.job_id, "test-id");
        assert_eq!(job.status, FinetuneStatus::Pending);
        assert_eq!(job.total_epochs, 5);
        assert_eq!(job.current_epoch, 0);
        assert_eq!(job.progress, 0.0);
        assert_eq!(job.current_loss, None);
        assert_eq!(job.error_message, None);
    }

    #[test]
    fn test_finetune_job_progress_calculation() {
        let mut job = FinetuneJob::new("test-id".to_string(), 10);

        job.update_progress(3, Some(0.5));
        assert!((job.progress - 30.0).abs() < 0.01);

        job.update_progress(7, Some(0.2));
        assert!((job.progress - 70.0).abs() < 0.01);

        job.update_progress(10, Some(0.1));
        assert!((job.progress - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_manager_thread_safety() {
        use std::thread;

        let manager = FinetuneJobManager::new();
        let manager_clone = manager.clone();

        // Create jobs from multiple threads
        let handle1 = thread::spawn(move || {
            for _ in 0..10 {
                manager_clone.create_job(3);
            }
        });

        let manager_clone2 = manager.clone();
        let handle2 = thread::spawn(move || {
            for _ in 0..10 {
                manager_clone2.create_job(5);
            }
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        let jobs = manager.list_jobs();
        assert_eq!(jobs.len(), 20);
    }
}
