use crate::entity::identity::AIEntityIdentity;
use async_trait::async_trait;
use serde_json;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;
use uuid::Uuid;

/// 存储错误
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("实体未找到: {0}")]
    EntityNotFound(Uuid),
    
    #[error("目录创建失败: {0}")]
    DirectoryCreationFailed(String),
}

/// 实体存储 trait
#[async_trait]
pub trait EntityStorage: Send + Sync {
    /// 保存实体
    async fn save(&self, entity: &AIEntityIdentity) -> Result<(), StorageError>;
    
    /// 加载实体
    async fn load(&self, entity_id: Uuid) -> Result<AIEntityIdentity, StorageError>;
    
    /// 删除实体
    async fn delete(&self, entity_id: Uuid) -> Result<(), StorageError>;
    
    /// 列出所有实体
    async fn list_all(&self) -> Result<Vec<AIEntityIdentity>, StorageError>;
    
    /// 按状态列出实体
    async fn list_by_status(&self, status: crate::entity::status::EntityStatus) -> Result<Vec<AIEntityIdentity>, StorageError>;
    
    /// 检查实体是否存在
    async fn exists(&self, entity_id: Uuid) -> Result<bool, StorageError>;
}

/// 文件系统存储实现
pub struct FileSystemStorage {
    base_path: PathBuf,
}

impl FileSystemStorage {
    /// 创建新的文件系统存储
    pub fn new(base_path: impl AsRef<Path>) -> Result<Self, StorageError> {
        let base = base_path.as_ref().to_path_buf();
        
        // 确保目录存在
        std::fs::create_dir_all(&base)?;
        
        Ok(Self { base_path: base })
    }
    
    /// 获取实体文件路径
    fn entity_path(&self, entity_id: Uuid) -> PathBuf {
        self.base_path.join(format!("{}.json", entity_id))
    }
    
    /// 创建实体目录
    async fn create_entity_dirs(&self, entity: &AIEntityIdentity) -> Result<(), StorageError> {
        fs::create_dir_all(&entity.memory_root).await?;
        fs::create_dir_all(&entity.performance_root).await?;
        fs::create_dir_all(&entity.discipline_root).await?;
        Ok(())
    }
}

#[async_trait]
impl EntityStorage for FileSystemStorage {
    async fn save(&self, entity: &AIEntityIdentity) -> Result<(), StorageError> {
        // 创建实体目录
        self.create_entity_dirs(entity).await?;
        
        // 序列化并保存
        let path = self.entity_path(entity.entity_id);
        let json = serde_json::to_string_pretty(entity)?;
        fs::write(path, json).await?;
        
        Ok(())
    }
    
    async fn load(&self, entity_id: Uuid) -> Result<AIEntityIdentity, StorageError> {
        let path = self.entity_path(entity_id);
        
        if !path.exists() {
            return Err(StorageError::EntityNotFound(entity_id));
        }
        
        let json = fs::read_to_string(path).await?;
        let entity: AIEntityIdentity = serde_json::from_str(&json)?;
        
        Ok(entity)
    }
    
    async fn delete(&self, entity_id: Uuid) -> Result<(), StorageError> {
        let path = self.entity_path(entity_id);
        
        if path.exists() {
            fs::remove_file(path).await?;
        }
        
        Ok(())
    }
    
    async fn list_all(&self) -> Result<Vec<AIEntityIdentity>, StorageError> {
        let mut entities = Vec::new();
        
        let mut entries = fs::read_dir(&self.base_path).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                if let Ok(json) = fs::read_to_string(&path).await {
                    if let Ok(entity) = serde_json::from_str::<AIEntityIdentity>(&json) {
                        entities.push(entity);
                    }
                }
            }
        }
        
        Ok(entities)
    }
    
    async fn list_by_status(&self, status: crate::entity::status::EntityStatus) -> Result<Vec<AIEntityIdentity>, StorageError> {
        let all = self.list_all().await?;
        Ok(all.into_iter().filter(|e| e.status == status).collect())
    }
    
    async fn exists(&self, entity_id: Uuid) -> Result<bool, StorageError> {
        Ok(self.entity_path(entity_id).exists())
    }
}

/// 内存存储实现 (用于测试)
pub struct MemoryStorage {
    entities: std::sync::RwLock<HashMap<Uuid, AIEntityIdentity>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            entities: std::sync::RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EntityStorage for MemoryStorage {
    async fn save(&self, entity: &AIEntityIdentity) -> Result<(), StorageError> {
        let mut entities = self.entities.write().unwrap();
        entities.insert(entity.entity_id, entity.clone());
        Ok(())
    }
    
    async fn load(&self, entity_id: Uuid) -> Result<AIEntityIdentity, StorageError> {
        let entities = self.entities.read().unwrap();
        entities
            .get(&entity_id)
            .cloned()
            .ok_or(StorageError::EntityNotFound(entity_id))
    }
    
    async fn delete(&self, entity_id: Uuid) -> Result<(), StorageError> {
        let mut entities = self.entities.write().unwrap();
        entities.remove(&entity_id);
        Ok(())
    }
    
    async fn list_all(&self) -> Result<Vec<AIEntityIdentity>, StorageError> {
        let entities = self.entities.read().unwrap();
        Ok(entities.values().cloned().collect())
    }
    
    async fn list_by_status(&self, status: crate::entity::status::EntityStatus) -> Result<Vec<AIEntityIdentity>, StorageError> {
        let entities = self.entities.read().unwrap();
        Ok(entities
            .values()
            .filter(|e| e.status == status)
            .cloned()
            .collect())
    }
    
    async fn exists(&self, entity_id: Uuid) -> Result<bool, StorageError> {
        let entities = self.entities.read().unwrap();
        Ok(entities.contains_key(&entity_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::archive::Department;
    use crate::governance::Seat;
    
    #[tokio::test]
    async fn test_memory_storage() {
        let storage = MemoryStorage::new();
        
        // 创建实体
        let entity = AIEntityIdentity::new(
            "TestEntity".to_string(),
            Seat::Tianshu,
            Department::Governance,
        );
        let id = entity.entity_id;
        
        // 保存
        storage.save(&entity).await.unwrap();
        
        // 检查存在
        assert!(storage.exists(id).await.unwrap());
        
        // 加载
        let loaded = storage.load(id).await.unwrap();
        assert_eq!(loaded.name, "TestEntity");
        assert_eq!(loaded.entity_id, id);
        
        // 列出所有
        let all = storage.list_all().await.unwrap();
        assert_eq!(all.len(), 1);
        
        // 按状态列出
        let active = storage.list_by_status(crate::entity::status::EntityStatus::Candidate).await.unwrap();
        assert_eq!(active.len(), 1);
        
        // 删除
        storage.delete(id).await.unwrap();
        assert!(!storage.exists(id).await.unwrap());
    }
}
